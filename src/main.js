import "./styles.css";
import App from "./App.svelte";
// import tauriapi from '@tauri-apps/api';
// const { taurishell } = tauriapi.shell;

const app = new App({
  target: document.getElementById("app"),
});

// const command = Command.sidecar('binaries/xmentccnbg', []);
// const response = await command.execute();
// console.log(response);
export default app;
