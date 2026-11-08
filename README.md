# []

## Creator


```
## Request
GET /content/<content-id>/meta

## Response
{
  "name": ...,
  "tags": [...],
  "parts": {
    "script": {
      // Merged with the overall tags
      "tags": [...],
      "mime": ...,
    },
    ...
  }
}
```

```
## Request
GET /content/<content-id>/<content-part>/ 

## Response
RAW FILE
```

## Hub


