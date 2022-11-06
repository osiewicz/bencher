use std::{cmp::Ordering, iter::Sum, ops::Add};

use ordered_float::OrderedFloat;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{mean::Mean, median::Median};

#[derive(Debug, Copy, Clone, Default, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonMetric {
    pub value: OrderedFloat<f64>,
    pub lower_bound: Option<OrderedFloat<f64>>,
    pub upper_bound: Option<OrderedFloat<f64>>,
}

impl PartialEq for JsonMetric {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && option_eq(self.lower_bound, other.lower_bound)
            && option_eq(self.upper_bound, other.upper_bound)
    }
}

fn option_eq<T>(left: Option<T>, right: Option<T>) -> bool
where
    T: PartialEq,
{
    if let Some(left) = left {
        if let Some(right) = right {
            left == right
        } else {
            false
        }
    } else {
        right.is_none()
    }
}

impl PartialOrd for JsonMetric {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JsonMetric {
    fn cmp(&self, other: &Self) -> Ordering {
        let value_order = self.value.cmp(&other.value);
        if Ordering::Equal == value_order {
            let upper_order = self.upper_bound.cmp(&other.upper_bound);
            if Ordering::Equal == upper_order {
                self.lower_bound.cmp(&other.lower_bound)
            } else {
                upper_order
            }
        } else {
            value_order
        }
    }
}

impl Add for JsonMetric {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let value = self.value + other.value;
        let lower_bound = option_add(self.lower_bound, self.value, other.lower_bound, other.value);
        let upper_bound = option_add(self.upper_bound, self.value, other.upper_bound, other.value);
        Self {
            value,
            lower_bound,
            upper_bound,
        }
    }
}

fn option_add<T>(
    left_bound: Option<T>,
    left_value: T,
    right_bound: Option<T>,
    right_value: T,
) -> Option<T>
where
    T: Add<Output = T>,
{
    if let Some(left_bound) = left_bound {
        if let Some(right_bound) = right_bound {
            Some(left_bound + right_bound)
        } else {
            Some(left_bound + right_value)
        }
    } else {
        if let Some(right_bound) = right_bound {
            Some(left_value + right_bound)
        } else {
            None
        }
    }
}

impl Sum for JsonMetric {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::default(), |s, o| s + o)
    }
}

impl std::ops::Div<usize> for JsonMetric {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            value: self.value / rhs as f64,
            lower_bound: self.lower_bound.map(|b| b / rhs as f64),
            upper_bound: self.upper_bound.map(|b| b / rhs as f64),
        }
    }
}

impl Mean for JsonMetric {}

impl Median for JsonMetric {}