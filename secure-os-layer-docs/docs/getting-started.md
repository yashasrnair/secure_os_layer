---
id: getting-started
title: Getting Started
sidebar_label: Getting Started
---

Welcome to the **Secure OS Layer SDK** documentation!

This guide will help you quickly set up and start using the SDK.

## Installation

You can install the SDK via npm:

```bash
npm install secure-os-layer-sdk
```

## Basic Usage

Below is a simple example to get started:
```
import { SecureOSLayerSDK } from 'secure-os-layer-sdk';

const sdk = new SecureOSLayerSDK("http://127.0.0.1:8080", "my-app-id");

sdk.getStatus()
  .then(status => console.log("Status:", status))
  .catch(error => console.error("Error:", error));
```
## Environment Setup
Ensure that your Secure OS Layer backend is running and accessible at the specified URL.

For more details on API endpoints, please see the API Reference.