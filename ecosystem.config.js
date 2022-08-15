module.exports = {
  apps: [
    {
      name: "pokemon",
      script: "./target/release/pokemon-cards.exe",
      exec_interpreter: "none",
      exec_mode: "fork_mode",
    },
  ],
};
