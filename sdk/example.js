// example.js
import { SecureOSLayerSDK } from "./sdk/index.js";

const sdk = new SecureOSLayerSDK("http://127.0.0.1:8080", "my-app-id");

async function runExamples() {
  try {
    const status = await sdk.getStatus();
    console.log("Status:", status);

    const apps = await sdk.listApps();
    console.log("Registered Apps:", apps);

    if (apps.length > 0) {
      const installResponse = await sdk.installApp(apps[0].app_id);
      console.log("Install Response:", installResponse);
    }

    // Optionally, retrieve data by ID (replace with a valid UUID string)
    // const data = await sdk.getData("d290f1ee-6c54-4b01-90e6-d701748f0851");
    // console.log("Data:", data);
  } catch (error) {
    console.error("Error using SDK:", error);
  }
}

runExamples();
