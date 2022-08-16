module.exports = {
  apps: [
    {
      name: "pokemon",
      script: "./target/release/pokemon-cards --addr 0.0.0.0:8080",
      exec_interpreter: "none",
      exec_mode: "fork_mode",
    },
  ],
};
