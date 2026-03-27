#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
pub struct ProgressRecord {
    pub student: Address,
    pub course_id: Symbol,
    pub progress_pct: u32,
    pub completed: bool,
    pub timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    Progress(Address, Symbol),
    Locked, // reentrancy guard
}

// ── Reentrancy guard ─────────────────────────────────────────────────────────

fn acquire_lock(env: &Env) {
    let locked: bool = env
        .storage()
        .instance()
        .get(&DataKey::Locked)
        .unwrap_or(false);
    assert!(!locked, "reentrant call");
    env.storage().instance().set(&DataKey::Locked, &true);
}

fn release_lock(env: &Env) {
    env.storage().instance().set(&DataKey::Locked, &false);
}

// ── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct AnalyticsContract;

#[contractimpl]
impl AnalyticsContract {
    /// Record or update a student's course progress
    pub fn record_progress(
        env: Env,
        student: Address,
        course_id: Symbol,
        progress_pct: u32,
    ) {
        acquire_lock(&env);

        student.require_auth();
        assert!(progress_pct <= 100, "Progress must be 0-100");

        let record = ProgressRecord {
            student: student.clone(),
            course_id: course_id.clone(),
            progress_pct,
            completed: progress_pct == 100,
            timestamp: env.ledger().timestamp(),
        };

        env.storage()
            .instance()
            .set(&DataKey::Progress(student, course_id), &record);

        release_lock(&env);
    }

    /// Get a student's progress for a course
    pub fn get_progress(env: Env, student: Address, course_id: Symbol) -> Option<ProgressRecord> {
        env.storage()
            .instance()
            .get(&DataKey::Progress(student, course_id))
    }
}

mod tests;
