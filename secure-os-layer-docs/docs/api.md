---
id: api
title: API Reference
sidebar_label: API Reference
---

## Overview

The Secure OS Layer SDK provides the following methods:

- **getStatus()**  
  Returns the status of the Secure OS Layer.

- **listApps()**  
  Retrieves a list of registered apps.

- **installApp(appId)**  
  Installs an app by its ID.

- **getData(id)**  
  Retrieves data for the given ID.

## Method Details

### getStatus()

```js
sdk.getStatus().then(status => {
  console.log(status);
});
```
### Response
```json
{
  "status": "ok",
  "message": "Secure OS Layer is running"
}
```
## listApps()
```js
sdk.listApps().then(apps => {
  console.log(apps);
});
```
### Response
```json
{
  "message": "App installed"
}
```
## installApp(appId)
```js
sdk.installApp("my-app-id").then(response => {
  console.log(response);
});
```
### Response
```json
{
  "message": "App installed"
}
```
## getData(id)
```js
sdk.getData("some-uuid-string").then(data => {
  console.log(data);
});
```
### Response
```json
{
  "id": "some-uuid-string",
  "key": "exampleKey",
  "value": "exampleValue"
}
```
