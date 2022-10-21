use std::str::FromStr;

use bencher_json::{jwt::JsonWebToken, JsonNewToken, JsonToken};
use chrono::{DateTime, TimeZone, Utc};
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, SqliteConnection};
use uuid::Uuid;

use crate::{error::api_error, schema, schema::token as token_table, util::ApiContext, ApiError};

use super::{auth::AuthUser, QueryUser};

macro_rules! same_user {
    ($auth_user:ident, $rbac:expr, $user_id:expr) => {
        if !($auth_user.is_admin(&$rbac) || $auth_user.id == $user_id) {
            return Err(crate::error::ApiError::SameUser($auth_user.id, $user_id));
        }
    };
}

pub(crate) use same_user;

// ~365 days / years * 24 hours / day * 60 minutes / hour * 60 seconds / minute
#[cfg(not(debug_assert))]
const MAX_TTL: u64 = 365 * 24 * 60 * 60;

#[derive(Queryable)]
pub struct QueryToken {
    pub id: i32,
    pub uuid: String,
    pub user_id: i32,
    pub name: String,
    pub jwt: String,
    pub creation: i64,
    pub expiration: i64,
}

impl QueryToken {
    pub fn get_id(conn: &mut SqliteConnection, uuid: impl ToString) -> Result<i32, ApiError> {
        schema::token::table
            .filter(schema::token::uuid.eq(uuid.to_string()))
            .select(schema::token::id)
            .first(conn)
            .map_err(api_error!())
    }

    pub fn get_uuid(conn: &mut SqliteConnection, id: i32) -> Result<Uuid, ApiError> {
        let uuid: String = schema::token::table
            .filter(schema::token::id.eq(id))
            .select(schema::token::uuid)
            .first(conn)
            .map_err(api_error!())?;
        Uuid::from_str(&uuid).map_err(api_error!())
    }

    pub fn into_json(self, conn: &mut SqliteConnection) -> Result<JsonToken, ApiError> {
        let Self {
            id: _,
            uuid,
            user_id,
            name,
            jwt,
            creation,
            expiration,
        } = self;
        Ok(JsonToken {
            uuid: Uuid::from_str(&uuid).map_err(api_error!())?,
            user: QueryUser::get_uuid(conn, user_id)?,
            name,
            token: jwt,
            creation: to_date_time(creation)?,
            expiration: to_date_time(expiration)?,
        })
    }
}

pub fn to_date_time(timestamp: i64) -> Result<DateTime<Utc>, ApiError> {
    Utc.timestamp_opt(timestamp, 0)
        .single()
        .ok_or(ApiError::Timestamp(timestamp))
}

#[derive(Insertable)]
#[diesel(table_name = token_table)]
pub struct InsertToken {
    pub uuid: String,
    pub user_id: i32,
    pub name: String,
    pub jwt: String,
    pub creation: i64,
    pub expiration: i64,
}

impl InsertToken {
    pub fn from_json(
        api_context: &mut ApiContext,
        token: JsonNewToken,
        auth_user: &AuthUser,
    ) -> Result<Self, ApiError> {
        let JsonNewToken { user, name, ttl } = token;

        let query_user = QueryUser::from_resource_id(&mut api_context.database, &user)?;
        same_user!(auth_user, api_context.rbac, query_user.id);

        // Only in production, set a max TTL of approximately one year
        // Disabled in development to enable long lived testing tokens
        #[cfg(not(debug_assert))]
        if ttl > MAX_TTL {
            return Err(ApiError::MaxTtl {
                requested: ttl,
                max: MAX_TTL,
            });
        }

        let jwt = JsonWebToken::new_api_key(
            &api_context.secret_key.encoding,
            query_user.email,
            ttl as usize,
        )
        .map_err(api_error!())?;

        let token_data = jwt
            .validate_api_key(&api_context.secret_key.decoding)
            .map_err(api_error!())?;

        Ok(Self {
            uuid: Uuid::new_v4().to_string(),
            user_id: query_user.id,
            name,
            jwt: jwt.to_string(),
            creation: token_data.claims.iat as i64,
            expiration: token_data.claims.exp as i64,
        })
    }
}
