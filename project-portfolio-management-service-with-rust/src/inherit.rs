//! Inheritance down the plan tree: the pure rule shared by everything a plan can set for itself or inherit from the
//! nearest ancestor that does (ways of working, governance frameworks, tolerances). DB-free and deterministic.
//! Nothing inherited is stored on the child: it is resolved on read, so changing a programme changes every project
//! beneath it that has not said otherwise.

use serde::Serialize;
use uuid::Uuid;

/// Where a plan's value comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    /// The plan's own link.
    Own,
    /// The nearest ancestor's link.
    Inherited,
    /// Neither: nothing is declared.
    None,
}

/// The outcome of [`resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolution {
    /// Where it came from.
    pub source: Source,
    /// The value followed, when there is one.
    pub value: Option<Uuid>,
    /// The plan whose link it is (the plan itself for [`Source::Own`]).
    pub from_plan: Option<Uuid>,
}

/// Decide the value for `plan`: its own link, else the first ancestor with one, else none. `ancestors` is
/// nearest first, each with that plan's link (if any).
#[must_use]
pub fn resolve(plan: Uuid, own: Option<Uuid>, ancestors: &[(Uuid, Option<Uuid>)]) -> Resolution {
    if let Some(w) = own {
        return Resolution {
            source: Source::Own,
            value: Some(w),
            from_plan: Some(plan),
        };
    }
    ancestors
        .iter()
        .find_map(|(p, link)| link.map(|w| (*p, w)))
        .map_or(
            Resolution {
                source: Source::None,
                value: None,
                from_plan: None,
            },
            |(p, w)| Resolution {
                source: Source::Inherited,
                value: Some(w),
                from_plan: Some(p),
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plan_follows_its_own_then_the_nearest_ancestors_then_nothing() {
        let (plan, parent, grand) = (Uuid::from_u128(1), Uuid::from_u128(2), Uuid::from_u128(3));
        let (mine, theirs, top) = (
            Uuid::from_u128(10),
            Uuid::from_u128(11),
            Uuid::from_u128(12),
        );
        let own = resolve(plan, Some(mine), &[(parent, Some(theirs))]);
        assert_eq!(
            (own.source, own.value, own.from_plan),
            (Source::Own, Some(mine), Some(plan))
        );
        let near = resolve(plan, None, &[(parent, Some(theirs)), (grand, Some(top))]);
        assert_eq!(
            (near.source, near.value, near.from_plan),
            (Source::Inherited, Some(theirs), Some(parent))
        );
        let far = resolve(plan, None, &[(parent, None), (grand, Some(top))]);
        assert_eq!(
            (far.value, far.from_plan),
            (Some(top), Some(grand)),
            "skips an ancestor with none"
        );
        let none = resolve(plan, None, &[(parent, None), (grand, None)]);
        assert_eq!(
            (none.source, none.value, none.from_plan),
            (Source::None, None, None)
        );
    }
}
