use uuid::Uuid;

pub enum Stage {
    Submitted,
    Initialized,
    Judging,
    Completed,
}

pub enum State {
    CorrectAnswer,
    WrongAnswer, // TODO: implementation check-on-data
    CompilationError, // TODO: show compile error
    RuntimeError, // TODO: show runtime error
    TimeLimitExceeded, // TODO: implementation check-on-data
    FormatError, // TODO: TODO: implementation check-on-data
}

pub struct JudgeResult {
    state: State,
    // killobytes
    used_memories: u32,
    // milliseconds
    used_times: u32,
}

pub struct Answer {
    id: Uuid,
    user: Uuid,
    source: String,
    language: Uuid,
    stage: Stage,
    result: Option<JudgeResult>,
}