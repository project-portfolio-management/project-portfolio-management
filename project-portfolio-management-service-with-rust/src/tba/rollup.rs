//! Cross-plan rollup (spec §15 TBA-9).

use super::BTreeMap;

/// Hard cap on how deep a containment walk goes.
pub const MAX_ROLLUP_DEPTH: usize = 32;

/// Hard cap on how many plans one rollup covers.
pub const MAX_ROLLUP_NODES: usize = 500;

/// One plan reached by a rollup walk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RollupNode {
    /// The plan.
    pub pid: uuid::Uuid,
    /// Hops from the root; the root itself is `0`.
    pub depth: usize,
}

/// The result of walking a containment tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RollupWalk {
    /// The root and every descendant reached, breadth-first.
    pub nodes: Vec<RollupNode>,
    /// Whether a cap stopped the walk before it ran out of plans.
    pub truncated: bool,
    /// Plans reached by more than one path, or that pointed back into
    /// the tree. Non-zero means the containment data holds a cycle the
    /// write-path validation should have refused.
    pub revisits: usize,
}

/// Walk a containment tree breadth-first from `root`.
///
/// Pure over an adjacency map, so the caller loads the plans in **one**
/// query and this decides the shape — rather than a query per level,
/// which is the N+1 this shape exists to avoid.
///
/// Three bounds, and the reasons differ:
///
/// - **A visited set**, because a cycle in `parent_pid` would otherwise
///   revisit nodes and expand exponentially, not merely loop. The write
///   path refuses a cycle, but a rollup that *trusts* that is one bulk
///   import or one direct `UPDATE` away from hanging the service — and
///   `revisits` reports it rather than silently absorbing it.
/// - **A depth cap**, matching the existing ancestor walk's posture.
/// - **A node cap**, so one enormous portfolio cannot become an
///   unbounded response.
///
/// `truncated` says a cap fired, because a rollup that quietly covers
/// half an estate reads as if it covered all of it.
#[must_use]
pub fn walk_descendants(
    children: &BTreeMap<uuid::Uuid, Vec<uuid::Uuid>>,
    root: uuid::Uuid,
    max_nodes: usize,
    max_depth: usize,
) -> RollupWalk {
    let mut seen: std::collections::BTreeSet<uuid::Uuid> = std::collections::BTreeSet::new();
    let mut nodes: Vec<RollupNode> = Vec::new();
    let mut queue: std::collections::VecDeque<RollupNode> = std::collections::VecDeque::new();
    let mut revisits = 0usize;
    let mut truncated = false;

    queue.push_back(RollupNode {
        pid: root,
        depth: 0,
    });
    seen.insert(root);

    while let Some(node) = queue.pop_front() {
        if nodes.len() >= max_nodes {
            truncated = true;
            break;
        }
        nodes.push(node);
        if node.depth >= max_depth {
            // Deeper plans exist but are not walked; that is a cap
            // firing, not an exhausted tree.
            if children.get(&node.pid).is_some_and(|kids| !kids.is_empty()) {
                truncated = true;
            }
            continue;
        }
        for child in children.get(&node.pid).into_iter().flatten() {
            if seen.insert(*child) {
                queue.push_back(RollupNode {
                    pid: *child,
                    depth: node.depth + 1,
                });
            } else {
                revisits += 1;
            }
        }
    }
    if !queue.is_empty() {
        truncated = true;
    }

    RollupWalk {
        nodes,
        truncated,
        revisits,
    }
}
