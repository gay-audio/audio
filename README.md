# [NAME OF PRODUCT]

[NAME OF PRODUCT] is a confederated standard for creators to be able to control the content they host.

> [!IMPORTANT]  
> Not all of the things listed here are currently implemented. This stands mostly as a guide.

Contained in this repo are two rust binaries: `creator`, and `hub`.

## Creator

The `creator` binary is used to host content. It exposes a few key end points for frontends and content hubs to use to access content.

```
GET /content?by_name=<name> // Returns the metadata of all content, optionally filtered by query strings.
[
  {
    "id": ...,
    "name": ...,
    "published": ..., // Will be filtered to be true if queried without credentials
    "tags": [...],
    "parts": {
      "script": {
        ...
      }
    }
  },
  ...
]

GET /content/<uuid> // Returns the metadata associated with the uuid
{
  "id": ...
  "name": ...,
  "published": ..., // Will be filtered to be true if queried without credentials
  "tags": [...],
  "parts": {
    "script": {
      ...
    }
  }
},

GET /content/<uuid>/<part> // Returns the file from the part of the content. <part> can be any string
RAW FILE
```

So if one wanted to find a particular piece of content that they knew the name of this would be the order of endpoints to query

```rust
let content = get("www.audio.org/content?by_name=Super%20salacious%20time%20galore").await;

let ContentMetadata { id, ... } = content.first().unwrap(); // Good enough I suppose

// Have a read of the script to see if it interests me...
let script = get(format!("www.audio.org/content/{id}/script")).unwrap();
// Time to bust out the headphones
let audio = get(format!("www.audio.org/content/{id}/audio")).unwrap();
// And why not have a read of the lore afterwards
let lore = get(format!("www.audio.org/content/{id}/lore")).unwrap();
```

Do note: `script`, `audio`, and `lore` are not special cased, these can be any UTF-8 compliant strings. Want to create your own Wikipedia within this space? Go right ahead, I'm not a cop.

## Hub (In progress)

Content hubs are servers that are dedicated to the distribution of content from creators.

To this aim they set out content standards (What tags are and aren't allowed with what other tags, what tags are required), and then connect to individual creators networks, all content on creators networks that adheres to the content standard set out is shown (So Sapphic creators who make content for men can still join content hubs dedicated to lesbians, their content for men just won't be shown).

When registering with a content hub creators can choose to have their content be copied (the content hub keeps a record on their servers in case of outages) or only be shadowed (whenever content is requested from the content hub it redirects to the creators network).

There are two types of hubs: federations, and confederations. Federated content hubs allow creators to register accounts with them to upload their own content, confederated hubs do not allow this.

Content hubs can also connect to other content hubs. So for example you could have a content hub dedicated to Sapphic content, that is connected to multiple content hubs that are dedicated to specific niches of Lesbian content.

Importantly these connections are entirely voluntary and non-binding. If you find that a content hub that you are a part of is becoming increasing toxic, or is simply not your vibe anymore, you may at any time withdraw your connection and look to other places.
