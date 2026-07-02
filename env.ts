export const env = {
    ANDROID_LIB_NAME: "test-game",
    IOS_CARGO_PROFILE: "release",
    PROJECT_NAME: "TestGame",
    APP_NAME: "test-game",
    CARGO_PROFILE_FOR_PROFILING: "release-debug",
    // CARGO_PROFILE_FOR_PROFILING: "dev",
};

Object.assign(process.env, env);
