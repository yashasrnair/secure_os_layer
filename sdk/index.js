// sdk/index.js
import fetch from "node-fetch";

export class SecureOSLayerSDK {
  /**
   * Construct a new SDK instance.
   *
   * @param {string} baseUrl - The base URL of the secure OS layer API (e.g., "http://127.0.0.1:8080")
   * @param {string} appId - The X-App-ID value to use for authentication/permissions
   */
  constructor(baseUrl, appId) {
    this.baseUrl = baseUrl;
    this.appId = appId;
  }

  _getHeaders() {
    return {
      "Content-Type": "application/json",
      "X-App-ID": this.appId,
    };
  }

  async getStatus() {
    const response = await fetch(`${this.baseUrl}/status`, {
      headers: this._getHeaders(),
    });
    return response.json();
  }

  async listApps() {
    const response = await fetch(`${this.baseUrl}/apps`, {
      headers: this._getHeaders(),
    });
    return response.json();
  }

  async installApp(appId) {
    const response = await fetch(`${this.baseUrl}/install`, {
      method: "POST",
      headers: this._getHeaders(),
      body: JSON.stringify({ app_id: appId }),
    });
    return response.json();
  }

  async getData(id) {
    const response = await fetch(`${this.baseUrl}/data/${id}`, {
      headers: this._getHeaders(),
    });
    return response.json();
  }
}
