use std::sync::Arc;

use bencher_json::{JsonNewReport, JsonReport, ResourceId};
use diesel::{
    expression_methods::BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
};
use dropshot::{
    endpoint, HttpError, HttpResponseAccepted, HttpResponseHeaders, HttpResponseOk, Path,
    RequestContext, TypedBody,
};
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    model::{
        branch::QueryBranch,
        metrics::Metrics,
        project::QueryProject,
        report::{InsertReport, QueryReport},
        testbed::QueryTestbed,
        user::QueryUser,
        version::InsertVersion,
    },
    schema,
    util::{cors::get_cors, headers::CorsHeaders, http_error, map_http_error, Context},
};

#[derive(Deserialize, JsonSchema)]
pub struct GetLsParams {
    pub project: ResourceId,
}

#[endpoint {
    method = OPTIONS,
    path =  "/v0/projects/{project}/reports",
    tags = ["projects", "reports"]
}]
pub async fn dir_options(
    _rqctx: Arc<RequestContext<Context>>,
    _path_params: Path<GetLsParams>,
) -> Result<HttpResponseHeaders<HttpResponseOk<String>>, HttpError> {
    Ok(get_cors::<Context>())
}

#[endpoint {
    method = GET,
    path =  "/v0/projects/{project}/reports",
    tags = ["projects", "reports"]
}]
pub async fn get_ls(
    rqctx: Arc<RequestContext<Context>>,
    path_params: Path<GetLsParams>,
) -> Result<HttpResponseHeaders<HttpResponseOk<Vec<JsonReport>>, CorsHeaders>, HttpError> {
    let user_id = QueryUser::auth(&rqctx).await?;
    let path_params = path_params.into_inner();
    let project_id = QueryProject::connection(&rqctx, user_id, &path_params.project).await?;

    let context = &mut *rqctx.context().lock().await;
    let conn = &mut context.db;
    let json: Vec<JsonReport> = schema::report::table
        .left_join(schema::testbed::table.on(schema::report::testbed_id.eq(schema::testbed::id)))
        .filter(schema::testbed::project_id.eq(project_id))
        .select((
            schema::report::id,
            schema::report::uuid,
            schema::report::user_id,
            schema::report::version_id,
            schema::report::testbed_id,
            schema::report::adapter,
            schema::report::start_time,
            schema::report::end_time,
        ))
        .order(schema::report::start_time.desc())
        .load::<QueryReport>(conn)
        .map_err(map_http_error!("Failed to get reports."))?
        .into_iter()
        .filter_map(|query| query.to_json(conn).ok())
        .collect();

    Ok(HttpResponseHeaders::new(
        HttpResponseOk(json),
        CorsHeaders::new_pub("GET".into()),
    ))
}

#[endpoint {
    method = OPTIONS,
    path =  "/v0/reports",
    tags = ["reports"]
}]
pub async fn post_options(
    _rqctx: Arc<RequestContext<Context>>,
) -> Result<HttpResponseHeaders<HttpResponseOk<String>>, HttpError> {
    Ok(get_cors::<Context>())
}

#[endpoint {
    method = POST,
    path = "/v0/reports",
    tags = ["reports"]
}]
// For simplicity, his query makes the assumption that all posts are perfectly
// chronological. That is, a report will never be posted for X after Y has
// already been submitted when X really happened before Y. For implementing git
// bisect more complex logic will be required.
pub async fn post(
    rqctx: Arc<RequestContext<Context>>,
    body: TypedBody<JsonNewReport>,
) -> Result<HttpResponseHeaders<HttpResponseAccepted<JsonReport>, CorsHeaders>, HttpError> {
    let user_id = QueryUser::auth(&rqctx).await?;

    let json_report = body.into_inner();

    let context = &mut *rqctx.context().lock().await;
    let conn = &mut context.db;

    // Verify that the branch and testbed are part of the same project
    let branch_id = QueryBranch::get_id(conn, &json_report.branch)?;
    let testbed_id = QueryTestbed::get_id(conn, &json_report.testbed)?;
    let branch_project_id = schema::branch::table
        .filter(schema::branch::id.eq(&branch_id))
        .select(schema::branch::project_id)
        .first::<i32>(conn)
        .map_err(map_http_error!("Failed to create report."))?;
    let testbed_project_id = schema::testbed::table
        .filter(schema::testbed::id.eq(&testbed_id))
        .select(schema::testbed::project_id)
        .first::<i32>(conn)
        .map_err(map_http_error!("Failed to create report."))?;
    if branch_project_id != testbed_project_id {
        return Err(http_error!("Failed to create report."));
    }
    let project_id = branch_project_id;

    // Verify that the user has access to the project
    QueryUser::has_access(conn, user_id, project_id)?;

    // If there is a hash then try to see if there is already a code version for
    // this branch with that particular hash.
    // Otherwise, create a new code version for this branch with/without the hash.
    let version_id = if let Some(hash) = &json_report.hash {
        if let Ok(version_id) = schema::version::table
            .filter(
                schema::version::branch_id
                    .eq(branch_id)
                    .and(schema::version::hash.eq(hash)),
            )
            .select(schema::version::id)
            .first::<i32>(conn)
        {
            version_id
        } else {
            InsertVersion::increment(conn, branch_id, Some(hash.clone()))?
        }
    } else {
        InsertVersion::increment(conn, branch_id, None)?
    };

    // Create a new report and add it to the database
    let insert_report = InsertReport::from_json(user_id, version_id, testbed_id, &json_report)?;

    diesel::insert_into(schema::report::table)
        .values(&insert_report)
        .execute(conn)
        .map_err(map_http_error!("Failed to create report."))?;

    let query_report = schema::report::table
        .filter(schema::report::uuid.eq(&insert_report.uuid))
        .first::<QueryReport>(conn)
        .map_err(map_http_error!("Failed to create report."))?;

    // Metrics is used to add benchmarks, perf metrics, and alerts.
    let mut metrics = Metrics::new(
        conn,
        project_id,
        branch_id,
        testbed_id,
        query_report.id,
        json_report.benchmarks.clone(),
    )?;

    for (index, benchmark) in json_report.benchmarks.inner.into_iter().enumerate() {
        for (benchmark_name, json_metrics) in benchmark.inner {
            metrics.benchmark(conn, index as i32, &benchmark_name, json_metrics)?;
        }
    }

    let json = query_report.to_json(conn)?;

    Ok(HttpResponseHeaders::new(
        HttpResponseAccepted(json),
        CorsHeaders::new_auth("POST".into()),
    ))
}

#[derive(Deserialize, JsonSchema)]
pub struct GetOneParams {
    pub project: ResourceId,
    pub report_uuid: Uuid,
}

#[endpoint {
    method = OPTIONS,
    path =  "/v0/projects/{project}/reports/{report_uuid}",
    tags = ["projects", "reports"]
}]
pub async fn one_options(
    _rqctx: Arc<RequestContext<Context>>,
    _path_params: Path<GetOneParams>,
) -> Result<HttpResponseHeaders<HttpResponseOk<String>>, HttpError> {
    Ok(get_cors::<Context>())
}

#[endpoint {
    method = GET,
    path =  "/v0/projects/{project}/reports/{report_uuid}",
    tags = ["projects", "reports"]
}]
pub async fn get_one(
    rqctx: Arc<RequestContext<Context>>,
    path_params: Path<GetOneParams>,
) -> Result<HttpResponseHeaders<HttpResponseOk<JsonReport>, CorsHeaders>, HttpError> {
    let user_id = QueryUser::auth(&rqctx).await?;
    let path_params = path_params.into_inner();
    let project_id = QueryProject::connection(&rqctx, user_id, &path_params.project).await?;
    let report_uuid = path_params.report_uuid.to_string();

    let context = &mut *rqctx.context().lock().await;
    let conn = &mut context.db;
    let json = schema::report::table
        .left_join(schema::testbed::table.on(schema::report::testbed_id.eq(schema::testbed::id)))
        .filter(
            schema::testbed::project_id
                .eq(project_id)
                .and(schema::report::uuid.eq(report_uuid)),
        )
        .select((
            schema::report::id,
            schema::report::uuid,
            schema::report::user_id,
            schema::report::version_id,
            schema::report::testbed_id,
            schema::report::adapter,
            schema::report::start_time,
            schema::report::end_time,
        ))
        .first::<QueryReport>(conn)
        .map_err(map_http_error!("Failed to get report."))?
        .to_json(conn)?;

    Ok(HttpResponseHeaders::new(
        HttpResponseOk(json),
        CorsHeaders::new_pub("GET".into()),
    ))
}