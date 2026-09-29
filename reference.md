# Reference
## Locations
<details><summary><code>client.locations.<a href="/src/api/resources/locations/client.rs">list_practice_locations</a>(practice_id: String, limit: Option&lt;Option&lt;i64&gt;&gt;, starting_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, ending_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, status: Option&lt;Option&lt;Option&lt;ListPracticeLocationsRequestStatus&gt;&gt;&gt;) -> Result&lt;ListPracticeLocationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires locations:read on a practice key or an authorized platform key. Lists active and archived locations by name, with cursor pagination. Use status to filter. Location records are shared between Test and Live for the same practice.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .locations
        .list_practice_locations(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &ListPracticeLocationsQueryRequest {
                starting_after: Some("loc_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ending_before: Some("loc_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**ending_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<ListPracticeLocationsRequestStatus>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.locations.<a href="/src/api/resources/locations/client.rs">create_practice_location</a>(practice_id: String, request: CreatePracticeLocationRequest) -> Result&lt;CreatePracticeLocationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires locations:write and Idempotency-Key for API keys. Creates an active location with a unique name in this practice. Locations are shared between Test and Live. Use the returned ID for Team location access.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .locations
        .create_practice_location(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &CreatePracticeLocationRequest {
                name: "name".to_string(),
                city: None,
                country: None,
                line1: None,
                line2: None,
                phone: None,
                postal_code: None,
                state: None,
                timezone: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**city:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**country:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**line1:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**line2:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**name:** `String`

</dd>
</dl>

<dl>
<dd>

**phone:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**postal_code:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**state:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<Option<String>>` — Optional IANA timezone override. Omit to leave unchanged; null clears it. No timezone is inferred when creating a record.

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.locations.<a href="/src/api/resources/locations/client.rs">get_practice_location</a>(practice_id: String, location_id: String) -> Result&lt;GetPracticeLocationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires locations:read. Returns one active or archived location in the authorized practice.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .locations
        .get_practice_location(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"loc_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**location_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.locations.<a href="/src/api/resources/locations/client.rs">update_practice_location</a>(practice_id: String, location_id: String, request: UpdatePracticeLocationRequest) -> Result&lt;UpdatePracticeLocationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires locations:write and Idempotency-Key for API keys. Updates only supplied fields; null clears optional contact and address fields. Archived locations cannot be updated. Changes apply to both Test and Live.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .locations
        .update_practice_location(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"loc_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &UpdatePracticeLocationRequest {
                ..Default::default()
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**location_id:** `String`

</dd>
</dl>

<dl>
<dd>

**city:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**country:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**line1:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**line2:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**name:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**phone:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**postal_code:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**state:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<Option<String>>` — Optional IANA timezone override. Omit to leave unchanged; null clears it. No timezone is inferred when creating a record.

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.locations.<a href="/src/api/resources/locations/client.rs">archive_practice_location</a>(practice_id: String, location_id: String) -> Result&lt;ArchivePracticeLocationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires locations:write and Idempotency-Key for API keys. Retains the location and historical associations. Archived locations cannot receive new Team assignments. Repeating archive returns the archived location. Changes apply to both Test and Live.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .locations
        .archive_practice_location(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"loc_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**location_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## API Keys
<details><summary><code>client.api_keys.<a href="/src/api/resources/api_keys/client.rs">create_platform_practice_api_key</a>(practice_id: String, request: CreatePlatformPracticeApiKeyRequest) -> Result&lt;CreatePlatformPracticeApiKeyResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a practice API key for a connected practice. Requires a platform key with service_keys:write and every requested scope. The practice key uses the platform key's Test or Live mode and cannot outlive it. Requires Idempotency-Key for safe retries; the secret is returned in the encrypted replay response for 24 hours.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .api_keys
        .create_platform_practice_api_key(
            &"practiceId".to_string(),
            &CreatePlatformPracticeAPIKeyRequest {
                name: "name".to_string(),
                allowed_ips: None,
                expires_at: None,
                scopes: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**allowed_ips:** `Option<Option<Vec<String>>>`

</dd>
</dl>

<dl>
<dd>

**expires_at:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**name:** `String`

</dd>
</dl>

<dl>
<dd>

**scopes:** `Option<Option<Vec<CreatePlatformPracticeApiKeyRequestScopesItem>>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api_keys.<a href="/src/api/resources/api_keys/client.rs">get_api_access</a>() -> Result&lt;GetApiAccessResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the subject, mode, and scopes for the API key.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.api_keys.get_api_access(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Account
<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">get_account</a>(org_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;GetAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the platform organization, request livemode, and effective access. API keys report scopes and the service_key role; dashboard sessions report membership permissions. operatingMode describes organization Live access, not the credential's Test/Live mode.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .get_account(
            &GetAccountQueryRequest {
                org_id: Some("acct_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `Option<Option<String>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Catalog
<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">list_catalog_items</a>(view: Option&lt;Option&lt;Option&lt;ListCatalogItemsRequestView&gt;&gt;&gt;, related_to_catalog_item_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, catalog_kind: Option&lt;Option&lt;Option&lt;ListCatalogItemsRequestCatalogKind&gt;&gt;&gt;, sort: Option&lt;Option&lt;Option&lt;ListCatalogItemsRequestSort&gt;&gt;&gt;, catalog_item_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, availability: Option&lt;Option&lt;Option&lt;ListCatalogItemsRequestAvailability&gt;&gt;&gt;, pharmacy_ids: Option&lt;Option&lt;Option&lt;ListCatalogItemsRequestPharmacyIds&gt;&gt;&gt;, dosage_forms: Option&lt;Option&lt;Option&lt;ListCatalogItemsRequestDosageForms&gt;&gt;&gt;, ending_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, hide_controlled_substances: Option&lt;Option&lt;bool&gt;&gt;, hide_unpriced: Option&lt;Option&lt;bool&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, org_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, practice_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, query: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, requirement: Option&lt;Option&lt;Option&lt;ListCatalogItemsRequestRequirement&gt;&gt;&gt;, routes: Option&lt;Option&lt;Option&lt;ListCatalogItemsRequestRoutes&gt;&gt;&gt;, starting_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;ListCatalogItemsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists catalog items for the authenticated account and mode. Use view=medications for priced prescription groups with offer counts, pharmacy counts, and strengths; the default view=offers returns individual offers. Use relatedToCatalogItemId to find offers for the same medication and route. When practiceId is supplied, a practice price overrides the platform price and missing overrides inherit the platform price.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .list_catalog_items(
            &ListCatalogItemsQueryRequest {
                related_to_catalog_item_id: Some("cat_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                catalog_item_id: Some("cat_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                pharmacy_ids: Some(ListCatalogItemsRequestPharmacyIDs::String(
                    "pharm_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                )),
                ending_before: Some("cat_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                org_id: Some("acct_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                practice_id: Some("prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                starting_after: Some("cat_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**view:** `Option<Option<ListCatalogItemsRequestView>>`

</dd>
</dl>

<dl>
<dd>

**related_to_catalog_item_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**catalog_kind:** `Option<Option<ListCatalogItemsRequestCatalogKind>>`

</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Option<ListCatalogItemsRequestSort>>`

</dd>
</dl>

<dl>
<dd>

**catalog_item_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**availability:** `Option<Option<ListCatalogItemsRequestAvailability>>`

</dd>
</dl>

<dl>
<dd>

**pharmacy_ids:** `Option<Option<ListCatalogItemsRequestPharmacyIds>>`

</dd>
</dl>

<dl>
<dd>

**dosage_forms:** `Option<Option<ListCatalogItemsRequestDosageForms>>`

</dd>
</dl>

<dl>
<dd>

**ending_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**hide_controlled_substances:** `Option<bool>`

</dd>
</dl>

<dl>
<dd>

**hide_unpriced:** `Option<bool>`

</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**org_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**practice_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**query:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**requirement:** `Option<Option<ListCatalogItemsRequestRequirement>>`

</dd>
</dl>

<dl>
<dd>

**routes:** `Option<Option<ListCatalogItemsRequestRoutes>>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<Option<String>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">list_pharmacies</a>(ending_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, org_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, pharmacy_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, query: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, ships_to_state: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, starting_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;ListPharmaciesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists pharmacies available to the authenticated account, including approved invite-only relationships.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .list_pharmacies(
            &ListPharmaciesQueryRequest {
                ending_before: Some("pharm_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                org_id: Some("acct_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                pharmacy_id: Some("pharm_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                starting_after: Some("pharm_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ending_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**org_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**pharmacy_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**query:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**ships_to_state:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<Option<String>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">list_shipping_options</a>(catalog_item_id: String, destination_state: Option&lt;String&gt;, destination_type: Option&lt;Option&lt;Option&lt;ListShippingOptionsRequestDestinationType&gt;&gt;&gt;) -> Result&lt;ListShippingOptionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns an array of at most 50 reviewed shipping services eligible for a catalog item, destination, and API mode. destinationState must be a USPS state or territory code. Each option has one temperature; pharmacy catalog summaries list all supported temperatures.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .list_shipping_options(
            &"cat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &ListShippingOptionsQueryRequest {
                destination_state: "destinationState".to_string(),
                destination_type: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**catalog_item_id:** `String`

</dd>
</dl>

<dl>
<dd>

**destination_state:** `String`

</dd>
</dl>

<dl>
<dd>

**destination_type:** `Option<Option<ListShippingOptionsRequestDestinationType>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">retrieve_prescribing_options</a>(catalog_item_id: String, practice_id: Option&lt;String&gt;) -> Result&lt;RetrievePrescribingOptionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires catalog:read. Returns reviewed SIG presets, guided patterns, quantity constraints and product requirements for a practice and mode. Revisions identify changed defaults. No patient-specific rationale or diagnosis is inferred.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .retrieve_prescribing_options(
            &"cat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &RetrievePrescribingOptionsQueryRequest {
                practice_id: "prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**catalog_item_id:** `String`

</dd>
</dl>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Orders
<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">list_orders</a>(query: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, external_order_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, created_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, created_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, ending_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, order_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, patient_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, patient_external_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, practice_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, sort: Option&lt;Option&lt;Option&lt;ListOrdersRequestSort&gt;&gt;&gt;, starting_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, status: Option&lt;Option&lt;Option&lt;ListOrdersRequestStatus&gt;&gt;&gt;) -> Result&lt;ListOrdersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .list_orders(
            &ListOrdersQueryRequest {
                ending_before: Some("ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                order_id: Some("ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                patient_id: Some("pat_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                practice_id: Some("prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                starting_after: Some("ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**query:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**external_order_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**ending_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**order_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**patient_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**patient_external_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**practice_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Option<ListOrdersRequestSort>>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<ListOrdersRequestStatus>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">create_order</a>(request: CreateOrderRequest) -> Result&lt;CreateOrderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates one unsigned order with 1–20 prescriptions for one patient in one practice. Supply patientId or patient; inline patient creation requires patients:write. Prescriber is optional: select by npi, provider id, or integration-scoped externalId, or leave the draft unassigned until signing. First-use prescriber registration requires team:write. Legacy userId is supported but cannot be combined with prescriber. Idempotency-Key is required.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .create_order(
            &CreateOrderRequest {
                practice_id: "prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                prescriptions: vec![CreateOrderRequestPrescriptionsItem {
                    external_prescription_id: None,
                    clinical: None,
                    pharmacy_id: None,
                    days_supply: 1,
                    dispensing: CreateOrderRequestPrescriptionsItemDispensing {
                        ..Default::default()
                    },
                    directions: "directions".to_string(),
                    medication_id: "cat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                    quantity: CreateOrderRequestPrescriptionsItemQuantity::Double(Infinity),
                    quantity_unit: "quantityUnit".to_string(),
                    refills: 1,
                    structured_sig: None,
                }],
                user_id: None,
                prescriber: None,
                otc_items: None,
                external_order_id: None,
                metadata: None,
                patient_id: None,
                patient: None,
                shipping_address_id: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**prescriber:** `Option<Option<CreateOrderRequestPrescriber>>`

</dd>
</dl>

<dl>
<dd>

**otc_items:** `Option<Option<Vec<CreateOrderRequestOtcItemsItem>>>`

</dd>
</dl>

<dl>
<dd>

**external_order_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, Option<CreateOrderRequestMetadataValue>>>>`

</dd>
</dl>

<dl>
<dd>

**patient_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**patient:** `Option<Option<CreateOrderRequestPatient>>`

</dd>
</dl>

<dl>
<dd>

**shipping_address_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**prescriptions:** `Vec<CreateOrderRequestPrescriptionsItem>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">get_order</a>(order_id: String) -> Result&lt;GetOrderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .get_order(&"ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">cancel_order</a>(order_id: String, request: CancelOrderRequest) -> Result&lt;CancelOrderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requests cancellation. HTTP 200 means the request was handled; check cancellation.status for confirmed, pending, partial, or failed. Only confirmed means the entire order is cancelled. Shipment possession makes a fulfillment cancellation too late.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .cancel_order(
            &"ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &CancelOrderRequest {
                reason: "reason".to_string(),
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String`

</dd>
</dl>

<dl>
<dd>

**reason:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">act_on_order_exception</a>(order_id: String, exception_id: String, request: ActOnOrderExceptionRequest) -> Result&lt;ActOnOrderExceptionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Acknowledge, retry, contact, or resolve an order exception in the credential's Test/Live mode. assign_to_me requires a signed-in dashboard user; API keys receive 400 and may use acknowledge instead. Actor headers do not create a dashboard assignee.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .act_on_order_exception(
            &"ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"fex_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &ActOnOrderExceptionRequest {
                action: ActOnOrderExceptionRequestAction::Acknowledge,
                note: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String`

</dd>
</dl>

<dl>
<dd>

**exception_id:** `String`

</dd>
</dl>

<dl>
<dd>

**action:** `ActOnOrderExceptionRequestAction`

</dd>
</dl>

<dl>
<dd>

**note:** `Option<Option<String>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">list_order_events</a>(order_id: String, ending_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, starting_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;ListOrderEventsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .list_order_events(
            &"ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &ListOrderEventsQueryRequest {
                ending_before: Some("evt_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                starting_after: Some("evt_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String`

</dd>
</dl>

<dl>
<dd>

**ending_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<Option<String>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">get_order_test_simulation</a>(order_id: String) -> Result&lt;GetOrderTestSimulationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires orders:write. Available only in Test mode.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .get_order_test_simulation(&"ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">update_order_test_simulation</a>(order_id: String, request: UpdateOrderTestSimulationRequest) -> Result&lt;UpdateOrderTestSimulationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires orders:write and Idempotency-Key. Configure before submission or queue a valid pharmacy event in manual mode. Events use normal order history and Test webhooks. Live requests are rejected.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .update_order_test_simulation(
            &"ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &UpdateOrderTestSimulationRequest {
                mode: UpdateOrderTestSimulationRequestMode::Automatic,
                scenario: UpdateOrderTestSimulationRequestScenario::Successful,
                action: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String`

</dd>
</dl>

<dl>
<dd>

**mode:** `UpdateOrderTestSimulationRequestMode`

</dd>
</dl>

<dl>
<dd>

**scenario:** `UpdateOrderTestSimulationRequestScenario`

</dd>
</dl>

<dl>
<dd>

**action:** `Option<Option<UpdateOrderTestSimulationRequestAction>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">preview_order</a>(request: PreviewOrderRequest) -> Result&lt;PreviewOrderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires orders:write and catalog:read. Supply exactly one of patientId, patientExternalId, or inline patient details. External-ID lookup additionally requires patients:read; inline details require patients:write. Resolves defaults and explicit edits for 1–20 prescriptions. Reuses stored patient details when identifiers match; otherwise previews inline details without creating a patient. Complete previews contain an orders.create input. Does not create records, reserve prices, sign, charge or transmit. No idempotency key is required. Creation and signing recheck current requirements.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .preview_order(
            &PreviewOrderRequest {
                practice_id: "prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                prescriptions: vec![PreviewOrderRequestPrescriptionsItem {
                    medication_id: "cat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                    ..Default::default()
                }],
                otc_items: None,
                patient_id: None,
                patient_external_id: None,
                patient: None,
                user_id: None,
                prescriber: None,
                shipping_address_id: None,
                external_order_id: None,
                shipping: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**otc_items:** `Option<Option<Vec<PreviewOrderRequestOtcItemsItem>>>`

</dd>
</dl>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**patient_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**patient_external_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**patient:** `Option<Option<PreviewOrderRequestPatient>>`

</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**prescriber:** `Option<Option<PreviewOrderRequestPrescriber>>`

</dd>
</dl>

<dl>
<dd>

**shipping_address_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**external_order_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**prescriptions:** `Vec<PreviewOrderRequestPrescriptionsItem>`

</dd>
</dl>

<dl>
<dd>

**shipping:** `Option<Option<PreviewOrderRequestShipping>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">sign_order</a>(order_id: String, request: SignOrderRequest) -> Result&lt;SignOrderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires orders:sign, Idempotency-Key, signatureAttestation, and expectedRevision from the reviewed order. Existing integrations may send expectedVersions instead; supply exactly one. A stale revision returns 409 and requires renewed clinician review. Select prescriber by npi, provider id, or integration-scoped externalId, or inherit the draft's prescriber. First-use registration requires team:write. Actor headers are optional audit metadata with prescriber; legacy userId requires matching clinician actor headers. Signing does not submit to a pharmacy.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .sign_order(
            &"ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &SignOrderRequest {
                practice_id: "prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                signature_attestation: true,
                user_id: None,
                prescriber: None,
                expected_revision: None,
                expected_versions: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String`

</dd>
</dl>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**prescriber:** `Option<Option<SignOrderRequestPrescriber>>`

</dd>
</dl>

<dl>
<dd>

**signature_attestation:** `bool`

</dd>
</dl>

<dl>
<dd>

**expected_revision:** `Option<Option<String>>` — Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.

</dd>
</dl>

<dl>
<dd>

**expected_versions:** `Option<Option<Vec<SignOrderRequestExpectedVersionsItem>>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">sign_and_submit_order</a>(order_id: String, request: SignAndSubmitOrderRequest) -> Result&lt;SignAndSubmitOrderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires orders:sign, Idempotency-Key, signatureAttestation, and expectedRevision from the reviewed order. Existing integrations may send expectedVersions instead; supply exactly one. A stale revision returns 409 and requires renewed clinician review. Select prescriber by npi, provider id, or externalId, or inherit the draft's prescriber. First-use registration requires team:write. Actor headers are optional with prescriber; legacy userId requires matching clinician actor headers. Signs the complete order, then attempts each submission. Signing remains committed if submission fails. Replay the same key after an uncertain response; retry reported submission failures through Submit order with a new key. Submitted means queued, not pharmacy acceptance.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .sign_and_submit_order(
            &"ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &SignAndSubmitOrderRequest {
                practice_id: "prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                signature_attestation: true,
                user_id: None,
                prescriber: None,
                expected_revision: None,
                expected_versions: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String`

</dd>
</dl>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**prescriber:** `Option<Option<SignAndSubmitOrderRequestPrescriber>>`

</dd>
</dl>

<dl>
<dd>

**signature_attestation:** `bool`

</dd>
</dl>

<dl>
<dd>

**expected_revision:** `Option<Option<String>>` — Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.

</dd>
</dl>

<dl>
<dd>

**expected_versions:** `Option<Option<Vec<SignAndSubmitOrderRequestExpectedVersionsItem>>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">submit_order</a>(order_id: String, request: SubmitOrderRequest) -> Result&lt;SubmitOrderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires orders:sign and Idempotency-Key. Queues signed prescriptions after rechecking authorization, signature integrity, billing, and fulfillment eligibility. Track pharmacy acceptance through order reads and webhooks. After a partial failure, retry submission with a new idempotency key; already queued prescriptions are not duplicated.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .submit_order(
            &"ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &SubmitOrderRequest {
                practice_id: "prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                user_id: None,
                prescriber: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String`

</dd>
</dl>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**prescriber:** `Option<Option<SubmitOrderRequestPrescriber>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">reject_order</a>(order_id: String, request: RejectOrderRequest) -> Result&lt;RejectOrderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires orders:sign and Idempotency-Key. Select a prescriber or inherit the draft's prescriber. Legacy userId requires matching clinician actor headers. Supply expectedRevision from the reviewed order, or expectedVersions for existing integrations. Permanently rejects the complete unsigned order after checking its revision.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .reject_order(
            &"ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &RejectOrderRequest {
                practice_id: "prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                reason: "reason".to_string(),
                user_id: None,
                prescriber: None,
                expected_revision: None,
                expected_versions: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String`

</dd>
</dl>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**prescriber:** `Option<Option<RejectOrderRequestPrescriber>>`

</dd>
</dl>

<dl>
<dd>

**reason:** `String`

</dd>
</dl>

<dl>
<dd>

**expected_revision:** `Option<Option<String>>` — Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.

</dd>
</dl>

<dl>
<dd>

**expected_versions:** `Option<Option<Vec<RejectOrderRequestExpectedVersionsItem>>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">add_order_prescription</a>(order_id: String, request: AddOrderPrescriptionRequest) -> Result&lt;AddOrderPrescriptionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires orders:write, Idempotency-Key and expectedRevision from the order being edited. Existing integrations may send expectedVersions instead; supply exactly one. Adds a complete prescription to an unsigned Order and returns all new versions. Omitted actor context defaults to the authenticated service account as a system actor. Patient and prescriber attribution stay fixed. Signed orders cannot be amended through this endpoint. Signing and submission require orders:sign through their separate endpoints.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .add_order_prescription(
            &"ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &AddOrderPrescriptionRequest {
                practice_id: "prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                prescription: AddOrderPrescriptionRequestPrescription {
                    external_prescription_id: None,
                    clinical: None,
                    pharmacy_id: None,
                    days_supply: 1,
                    dispensing: AddOrderPrescriptionRequestPrescriptionDispensing {
                        ..Default::default()
                    },
                    directions: "directions".to_string(),
                    medication_id: "cat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                    quantity: AddOrderPrescriptionRequestPrescriptionQuantity::Double(Infinity),
                    quantity_unit: "quantityUnit".to_string(),
                    refills: 1,
                    structured_sig: None,
                },
                metadata: None,
                expected_revision: None,
                expected_versions: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String`

</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, Option<AddOrderPrescriptionRequestMetadataValue>>>>`

</dd>
</dl>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**expected_revision:** `Option<Option<String>>` — Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.

</dd>
</dl>

<dl>
<dd>

**expected_versions:** `Option<Option<Vec<AddOrderPrescriptionRequestExpectedVersionsItem>>>`

</dd>
</dl>

<dl>
<dd>

**prescription:** `AddOrderPrescriptionRequestPrescription`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">update_order_prescription</a>(order_id: String, prescription_id: String, request: UpdateOrderPrescriptionRequest) -> Result&lt;UpdateOrderPrescriptionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires orders:write, Idempotency-Key and expectedRevision from the order being edited. Existing integrations may send expectedVersions instead; supply exactly one. Replaces one prescription with complete medication instructions and returns all new versions. Omitted actor context defaults to the authenticated service account as a system actor. Patient and prescriber attribution stay fixed. Signed orders cannot be amended through this endpoint. Signing and submission require orders:sign through their separate endpoints.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .update_order_prescription(
            &"ord_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"rx_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &UpdateOrderPrescriptionRequest {
                practice_id: "prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                prescription: UpdateOrderPrescriptionRequestPrescription {
                    clinical: None,
                    pharmacy_id: None,
                    days_supply: 1,
                    dispensing: UpdateOrderPrescriptionRequestPrescriptionDispensing {
                        ..Default::default()
                    },
                    directions: "directions".to_string(),
                    medication_id: "cat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                    quantity: UpdateOrderPrescriptionRequestPrescriptionQuantity::Double(Infinity),
                    quantity_unit: "quantityUnit".to_string(),
                    refills: 1,
                    structured_sig: None,
                },
                metadata: None,
                expected_revision: None,
                expected_versions: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String`

</dd>
</dl>

<dl>
<dd>

**prescription_id:** `String`

</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, Option<UpdateOrderPrescriptionRequestMetadataValue>>>>`

</dd>
</dl>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**expected_revision:** `Option<Option<String>>` — Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.

</dd>
</dl>

<dl>
<dd>

**expected_versions:** `Option<Option<Vec<UpdateOrderPrescriptionRequestExpectedVersionsItem>>>`

</dd>
</dl>

<dl>
<dd>

**prescription:** `UpdateOrderPrescriptionRequestPrescription`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.orders.<a href="/src/api/resources/orders/client.rs">create_order_batch</a>(request: CreateOrderBatchRequest) -> Result&lt;CreateOrderBatchResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates 1–20 orders for distinct patients in one practice, each with 1–20 prescriptions. Each accepts patientId or inline patient details. Orders and newly created patients commit atomically; any failure saves none. Requires orders:write and Idempotency-Key; inline patients also require patients:write. Omitted actor context defaults to the authenticated service account as a system actor. Sign and submit each resulting order separately using orders:sign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .orders
        .create_order_batch(
            &CreateOrderBatchRequest {
                practice_id: "prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                orders: vec![CreateOrderBatchRequestOrdersItem {
                    prescriptions: vec![CreateOrderBatchRequestOrdersItemPrescriptionsItem {
                        external_prescription_id: None,
                        clinical: None,
                        pharmacy_id: None,
                        days_supply: 1,
                        dispensing: CreateOrderBatchRequestOrdersItemPrescriptionsItemDispensing {
                            ..Default::default()
                        },
                        directions: "directions".to_string(),
                        medication_id: "cat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
                        quantity:
                            CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantity::Double(
                                Infinity,
                            ),
                        quantity_unit: "quantityUnit".to_string(),
                        refills: 1,
                        structured_sig: None,
                    }],
                    ..Default::default()
                }],
                user_id: None,
                prescriber: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**prescriber:** `Option<Option<CreateOrderBatchRequestPrescriber>>`

</dd>
</dl>

<dl>
<dd>

**orders:** `Vec<CreateOrderBatchRequestOrdersItem>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Webhooks
<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">list_webhook_endpoints</a>(ending_before: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, starting_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListWebhookEndpointsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires webhooks:read. Returns endpoints owned by the key organization, or the organization selected with X-Affinity-Organization-Id. Platform delegation requires a webhook grant in the key's mode.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .list_webhook_endpoints(
            &ListWebhookEndpointsQueryRequest {
                ending_before: Some("whe_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                starting_after: Some("whe_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ending_before:** `Option<String>`

</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<String>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">create_webhook_endpoint</a>(request: CreateWebhookEndpointRequest) -> Result&lt;CreateWebhookEndpointResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires webhooks:write and Idempotency-Key. Defaults to the API key organization. A platform can select a practice or pharmacy owner with X-Affinity-Organization-Id and an explicit webhook grant. For platform-owned endpoints, practiceIds narrows delivery to selected connected practices. An empty filter receives all otherwise-authorized events.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .create_webhook_endpoint(
            &CreateWebhookEndpointRequest {
                url: "url".to_string(),
                practice_ids: None,
                description: None,
                payload_style: None,
                subscribed_events: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_ids:** `Option<Vec<String>>`

</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>`

</dd>
</dl>

<dl>
<dd>

**payload_style:** `Option<CreateWebhookEndpointRequestPayloadStyle>`

</dd>
</dl>

<dl>
<dd>

**subscribed_events:** `Option<Vec<CreateWebhookEndpointRequestSubscribedEventsItem>>`

</dd>
</dl>

<dl>
<dd>

**url:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">delete_webhook_endpoint</a>(endpoint_id: String) -> Result&lt;DeleteWebhookEndpointResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .delete_webhook_endpoint(
            &"whe_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**endpoint_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">update_webhook_endpoint</a>(endpoint_id: String, request: UpdateWebhookEndpointRequest) -> Result&lt;UpdateWebhookEndpointResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires webhooks:write and Idempotency-Key. Updates an endpoint in the selected organization and mode. Omitted practiceIds preserves the filter; an empty array removes the practice filter. Subscription changes apply to newly generated events.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .update_webhook_endpoint(
            &"whe_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &UpdateWebhookEndpointRequest {
                ..Default::default()
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**endpoint_id:** `String`

</dd>
</dl>

<dl>
<dd>

**practice_ids:** `Option<Vec<String>>`

</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>`

</dd>
</dl>

<dl>
<dd>

**payload_style:** `Option<UpdateWebhookEndpointRequestPayloadStyle>`

</dd>
</dl>

<dl>
<dd>

**status:** `Option<UpdateWebhookEndpointRequestStatus>`

</dd>
</dl>

<dl>
<dd>

**subscribed_events:** `Option<Vec<UpdateWebhookEndpointRequestSubscribedEventsItem>>`

</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">rotate_webhook_endpoint_secret</a>(endpoint_id: String) -> Result&lt;RotateWebhookEndpointSecretResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .rotate_webhook_endpoint_secret(
            &"whe_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**endpoint_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">test_webhook_endpoint</a>(endpoint_id: String) -> Result&lt;TestWebhookEndpointResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .test_webhook_endpoint(
            &"whe_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**endpoint_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">list_webhook_events</a>(ending_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, status: Option&lt;Option&lt;Option&lt;ListWebhookEventsRequestStatus&gt;&gt;&gt;, starting_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;ListWebhookEventsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .list_webhook_events(
            &ListWebhookEventsQueryRequest {
                ending_before: Some("evt_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                starting_after: Some("evt_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ending_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<ListWebhookEventsRequestStatus>>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<Option<String>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">get_webhook_event</a>(event_id: String) -> Result&lt;GetWebhookEventResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .get_webhook_event(&"evt_01j2y8m6jcc9tt24af5pw9x1bc".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**event_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">replay_webhook_event</a>(event_id: String) -> Result&lt;ReplayWebhookEventResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .replay_webhook_event(
            &"evt_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**event_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">list_webhook_grants</a>(limit: Option&lt;Option&lt;i64&gt;&gt;, starting_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, ending_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;ListWebhookGrantsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires webhooks:read on the owning practice or pharmacy key. Lists platform webhook grants in the key's mode. Platforms cannot list or grant themselves delegated access.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .list_webhook_grants(
            &ListWebhookGrantsQueryRequest {
                starting_after: Some("acct_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ending_before: Some("acct_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**ending_before:** `Option<Option<String>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">save_webhook_grant</a>(platform_id: String, request: SaveWebhookGrantRequest) -> Result&lt;SaveWebhookGrantResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires webhooks:write on the owning practice or pharmacy key and Idempotency-Key. Grants or replaces a platform's webhook permissions in this mode. A practice must already be connected to that platform. The grant does not give the platform access to other API resources.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .save_webhook_grant(
            &"acct_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &SaveWebhookGrantRequest {
                scopes: vec![SaveWebhookGrantRequestScopesItem::WebhooksRead],
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**platform_id:** `String`

</dd>
</dl>

<dl>
<dd>

**scopes:** `Vec<SaveWebhookGrantRequestScopesItem>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">revoke_webhook_grant</a>(platform_id: String) -> Result&lt;RevokeWebhookGrantResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires webhooks:write on the owning practice or pharmacy key and Idempotency-Key. Removes platform webhook access in this mode. Existing endpoints remain owned by the practice or pharmacy and continue operating.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .revoke_webhook_grant(
            &"acct_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**platform_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Team
<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">register_user</a>(practice_id: String, request: RegisterUserRequest) -> Result&lt;RegisterUserResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:write and Idempotency-Key. Registers a practice member without an invitation. Test requires synthetic .test emails and Affinity Test NPIs. Live requires approved integration and practice access. Identity attestation records the integration's assertion; it does not verify login email or clinical credentials. Existing memberships and verified provider records are preserved. Use the returned user ID for orders and signing.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .register_user(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &RegisterUserRequest {
                external_id: "externalId".to_string(),
                email: "email".to_string(),
                name: "name".to_string(),
                role: RegisterUserRequestRole::Administrator,
                identity_attestation: true,
                roles: None,
                profile_details: None,
                npi: None,
                licenses: None,
                legal_name: None,
                display_name: None,
                credentials: None,
                address: None,
                phone: None,
                location_ids: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**external_id:** `String`

</dd>
</dl>

<dl>
<dd>

**email:** `String`

</dd>
</dl>

<dl>
<dd>

**name:** `String`

</dd>
</dl>

<dl>
<dd>

**role:** `RegisterUserRequestRole`

</dd>
</dl>

<dl>
<dd>

**roles:** `Option<Option<Vec<RegisterUserRequestRolesItem>>>`

</dd>
</dl>

<dl>
<dd>

**profile_details:** `Option<Option<RegisterUserRequestProfileDetails>>`

</dd>
</dl>

<dl>
<dd>

**npi:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**licenses:** `Option<Option<Vec<RegisterUserRequestLicensesItem>>>`

</dd>
</dl>

<dl>
<dd>

**legal_name:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**display_name:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**credentials:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**address:** `Option<Option<RegisterUserRequestAddress>>`

</dd>
</dl>

<dl>
<dd>

**phone:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**location_ids:** `Option<Option<Vec<String>>>`

</dd>
</dl>

<dl>
<dd>

**identity_attestation:** `bool`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">list_practice_team_invitations</a>(practice_id: String, limit: Option&lt;Option&lt;i64&gt;&gt;, starting_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, ending_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, status: Option&lt;Option&lt;Option&lt;ListPracticeTeamInvitationsRequestStatus&gt;&gt;&gt;, email: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, external_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;ListPracticeTeamInvitationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:read. Lists practice invitations, including invitations sent in Clinic. Filter by pending, expired, accepted, declined, or revoked status, exact email, or your integration externalId. Only your integration and API key mode can see its external identity and onboarding state. Follow person.nextActions after invitation acceptance.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .list_practice_team_invitations(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &ListPracticeTeamInvitationsQueryRequest {
                starting_after: Some("invite_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ending_before: Some("invite_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**ending_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<ListPracticeTeamInvitationsRequestStatus>>`

</dd>
</dl>

<dl>
<dd>

**email:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**external_id:** `Option<Option<String>>` — Match this integration's external identity in the API key's mode.

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">invite_practice_team_person</a>(practice_id: String, request: InvitePracticeTeamPersonRequest) -> Result&lt;InvitePracticeTeamPersonResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:write on the practice key or its platform key. Use roles to combine administrator, prescriber, clinical_staff, billing, or developer presets. Ownership uses the protected owner designation. The singular role field remains available for single-role assignments. Creates a real organization invitation and optional prescriber setup. The recipient must accept with their Affinity account. Repeating the same external identity retries pending invitation delivery. Accepted invitations do not change existing access. Team membership is shared between Test and Live; the external identity is mode-scoped. Keys cannot accept invitations. Headless registration and signing use separate endpoints.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .invite_practice_team_person(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &InvitePracticeTeamPersonRequest {
                external_id: "externalId".to_string(),
                email: "email".to_string(),
                name: "name".to_string(),
                role: None,
                roles: None,
                profile_details: None,
                npi: None,
                licenses: None,
                legal_name: None,
                display_name: None,
                credentials: None,
                address: None,
                phone: None,
                location_ids: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**external_id:** `String`

</dd>
</dl>

<dl>
<dd>

**email:** `String`

</dd>
</dl>

<dl>
<dd>

**name:** `String`

</dd>
</dl>

<dl>
<dd>

**role:** `Option<Option<InvitePracticeTeamPersonRequestRole>>`

</dd>
</dl>

<dl>
<dd>

**roles:** `Option<Option<Vec<InvitePracticeTeamPersonRequestRolesItem>>>`

</dd>
</dl>

<dl>
<dd>

**profile_details:** `Option<Option<InvitePracticeTeamPersonRequestProfileDetails>>`

</dd>
</dl>

<dl>
<dd>

**npi:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**licenses:** `Option<Option<Vec<InvitePracticeTeamPersonRequestLicensesItem>>>`

</dd>
</dl>

<dl>
<dd>

**legal_name:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**display_name:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**credentials:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**address:** `Option<Option<InvitePracticeTeamPersonRequestAddress>>`

</dd>
</dl>

<dl>
<dd>

**phone:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**location_ids:** `Option<Option<Vec<String>>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">get_practice_team</a>(practice_id: String) -> Result&lt;GetPracticeTeamResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:read. Returns counts of members, invitations, and prescribers. Use the paginated members, prescribers, and invitations collections for individual records. Team access and clinician credentials are shared between Test and Live.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .get_practice_team(&"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">list_practice_team_members</a>(practice_id: String, limit: Option&lt;Option&lt;i64&gt;&gt;, starting_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, ending_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, search: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, role: Option&lt;Option&lt;Option&lt;ListPracticeTeamMembersRequestRole&gt;&gt;&gt;, status: Option&lt;Option&lt;Option&lt;ListPracticeTeamMembersRequestStatus&gt;&gt;&gt;) -> Result&lt;ListPracticeTeamMembersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:read. Search the roster by name or email, and filter by role or membership status. Includes members invited in Clinic, location access, and account-specific prescriber connections. Memberships are shared between Test and Live.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .list_practice_team_members(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &ListPracticeTeamMembersQueryRequest {
                starting_after: Some("mbr_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ending_before: Some("mbr_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**ending_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**search:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**role:** `Option<Option<ListPracticeTeamMembersRequestRole>>`

</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<ListPracticeTeamMembersRequestStatus>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">list_practice_team_prescribers</a>(practice_id: String, limit: Option&lt;Option&lt;i64&gt;&gt;, starting_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, ending_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, search: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, npi: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, state: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, status: Option&lt;Option&lt;Option&lt;ListPracticeTeamPrescribersRequestStatus&gt;&gt;&gt;) -> Result&lt;ListPracticeTeamPrescribersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:read. Filter practice prescribers by name, NPI, state, and practice status. Records include submitted licenses and their IDs. Signing authority also requires an active account connection, Live practice access, and prescription eligibility.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .list_practice_team_prescribers(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &ListPracticeTeamPrescribersQueryRequest {
                starting_after: Some("prov_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ending_before: Some("prov_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**ending_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**search:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**npi:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**state:** `Option<Option<String>>` — Match a submitted license jurisdiction. This does not establish signing eligibility.

</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<ListPracticeTeamPrescribersRequestStatus>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">get_practice_team_member</a>(practice_id: String, member_id: String) -> Result&lt;GetPracticeTeamMemberResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:read. Returns current account membership, roles, location access, and prescriber connection. The member ID identifies practice access; it is not the integration user ID used by orders.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .get_practice_team_member(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"mbr_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**member_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">update_practice_team_member</a>(practice_id: String, member_id: String, request: UpdatePracticeTeamMemberRequest) -> Result&lt;UpdatePracticeTeamMemberResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:write. Supply role, status, or locationIds; omitted values stay unchanged. A role replaces existing roles. Disable access with status disabled. An empty locationIds array grants all practice locations. Ownership changes require an active practice owner using a personal API key; service keys manage non-owner memberships. The final active owner cannot be removed. Changes apply to both Test and Live. Sign-in email and account security remain account settings.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .update_practice_team_member(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"mbr_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &UpdatePracticeTeamMemberRequest {
                ..Default::default()
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**member_id:** `String`

</dd>
</dl>

<dl>
<dd>

**role:** `Option<Option<UpdatePracticeTeamMemberRequestRole>>`

</dd>
</dl>

<dl>
<dd>

**roles:** `Option<Option<Vec<UpdatePracticeTeamMemberRequestRolesItem>>>`

</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<UpdatePracticeTeamMemberRequestStatus>>`

</dd>
</dl>

<dl>
<dd>

**location_ids:** `Option<Option<Vec<String>>>` — Replace location access. An empty array grants access to all practice locations.

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">get_practice_team_prescriber</a>(practice_id: String, prescriber_id: String) -> Result&lt;GetPracticeTeamPrescriberResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:read. Returns the clinical profile and submitted licenses, including license IDs. This is setup information, not a signing authorization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .get_practice_team_prescriber(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"prov_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**prescriber_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">update_practice_team_prescriber</a>(practice_id: String, prescriber_id: String, request: UpdatePracticeTeamPrescriberRequest) -> Result&lt;UpdatePracticeTeamPrescriberResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:write. Set practiceStatus to inactive to remove prescribing access in this practice, or active to restore an existing association. This does not create membership or signing authority. Practice status applies to Test and Live. Shared identity and license edits require Affinity support.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .update_practice_team_prescriber(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"prov_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &UpdatePracticeTeamPrescriberRequest {
                ..Default::default()
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**prescriber_id:** `String`

</dd>
</dl>

<dl>
<dd>

**display_name:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**legal_name:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**credentials:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**phone:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**address:** `Option<Option<UpdatePracticeTeamPrescriberRequestAddress>>`

</dd>
</dl>

<dl>
<dd>

**practice_status:** `Option<UpdatePracticeTeamPrescriberRequestPracticeStatus>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">create_practice_team_license</a>(practice_id: String, prescriber_id: String, request: CreatePracticeTeamLicenseRequest) -> Result&lt;CreatePracticeTeamLicenseResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:write and an active accepted prescriber account connection in this practice. Adds a license. Expiration is optional, but must be in the future when supplied. An exact repeat returns the existing license; update an existing license with PATCH and its license ID. Licenses are shared across practices and Test/Live. Other licenses stay unchanged.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .create_practice_team_license(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"prov_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &CreatePracticeTeamLicenseRequest {
                state: "state".to_string(),
                license_number: "licenseNumber".to_string(),
                expires_at: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**prescriber_id:** `String`

</dd>
</dl>

<dl>
<dd>

**state:** `String`

</dd>
</dl>

<dl>
<dd>

**license_number:** `String`

</dd>
</dl>

<dl>
<dd>

**expires_at:** `Option<Option<String>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">update_practice_team_license</a>(practice_id: String, prescriber_id: String, license_id: String, request: UpdatePracticeTeamLicenseRequest) -> Result&lt;UpdatePracticeTeamLicenseResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:write and an active accepted prescriber account connection in this practice. Correct the state or license number, or set or clear the optional expiresAt value. A supplied expiration must be in the future. Other licenses stay unchanged. Changes apply across practices and Test/Live.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .update_practice_team_license(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"prov_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"lic_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &UpdatePracticeTeamLicenseRequest {
                ..Default::default()
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**prescriber_id:** `String`

</dd>
</dl>

<dl>
<dd>

**license_id:** `String`

</dd>
</dl>

<dl>
<dd>

**state:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**license_number:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**expires_at:** `Option<Option<String>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">get_practice_team_invitation</a>(practice_id: String, invitation_id: String) -> Result&lt;GetPracticeTeamInvitationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:read. Returns invitation status and current onboarding state for your integration. An accepted invitation can still have disabled membership or pending clinical review. Invitation tokens are never returned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .get_practice_team_invitation(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"invite_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**invitation_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">revoke_practice_team_invitation</a>(practice_id: String, invitation_id: String) -> Result&lt;RevokePracticeTeamInvitationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:write. Revokes a pending or expired invitation and its pending prescriber account connection. Repeating the revoke returns the revoked invitation. Accepted invitations return 409; disable the member instead. Retains invitation history.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .revoke_practice_team_invitation(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"invite_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**invitation_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team.<a href="/src/api/resources/team/client.rs">resend_practice_team_invitation</a>(practice_id: String, invitation_id: String) -> Result&lt;ResendPracticeTeamInvitationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires team:write. Resends a pending or expired invitation with the same ID, recipient, roles, and locations. The previous link stops working and the new link expires in seven days. Accepted and revoked invitations return 409. A 502 means the invitation was saved but email delivery could not be confirmed; retry this operation.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .team
        .resend_practice_team_invitation(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"invite_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**invitation_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Patients
<details><summary><code>client.patients.<a href="/src/api/resources/patients/client.rs">list_patient_addresses</a>(practice_id: String, patient_id: String, status: Option&lt;Option&lt;Option&lt;ListPatientAddressesRequestStatus&gt;&gt;&gt;, starting_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, ending_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListPatientAddressesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .patients
        .list_patient_addresses(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"pat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &ListPatientAddressesQueryRequest {
                starting_after: Some("addr_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ending_before: Some("addr_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**patient_id:** `String`

</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<ListPatientAddressesRequestStatus>>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**ending_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.patients.<a href="/src/api/resources/patients/client.rs">create_patient_address</a>(practice_id: String, patient_id: String, request: CreatePatientAddressRequest) -> Result&lt;CreatePatientAddressResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the existing active address for a normalized duplicate. The first address becomes the default. API keys require Idempotency-Key.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .patients
        .create_patient_address(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"pat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &CreatePatientAddressRequest {
                address: CreatePatientAddressRequestAddress {
                    city: "city".to_string(),
                    line1: "line1".to_string(),
                    postal_code: "postalCode".to_string(),
                    state: "state".to_string(),
                    ..Default::default()
                },
                label: None,
                preferred_shipping: None,
                recipient_name: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**patient_id:** `String`

</dd>
</dl>

<dl>
<dd>

**address:** `CreatePatientAddressRequestAddress`

</dd>
</dl>

<dl>
<dd>

**label:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**preferred_shipping:** `Option<Option<bool>>`

</dd>
</dl>

<dl>
<dd>

**recipient_name:** `Option<Option<String>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.patients.<a href="/src/api/resources/patients/client.rs">archive_patient_address</a>(practice_id: String, patient_id: String, address_id: String) -> Result&lt;ArchivePatientAddressResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Preserves the address ID and history. Archiving the default selects the oldest remaining active address. Existing orders remain unchanged.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .patients
        .archive_patient_address(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"pat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"addr_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**patient_id:** `String`

</dd>
</dl>

<dl>
<dd>

**address_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.patients.<a href="/src/api/resources/patients/client.rs">update_patient_address</a>(practice_id: String, patient_id: String, address_id: String, request: UpdatePatientAddressRequest) -> Result&lt;UpdatePatientAddressResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .patients
        .update_patient_address(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"pat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"addr_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &UpdatePatientAddressRequest {
                ..Default::default()
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**patient_id:** `String`

</dd>
</dl>

<dl>
<dd>

**address_id:** `String`

</dd>
</dl>

<dl>
<dd>

**address:** `Option<Option<UpdatePatientAddressRequestAddress>>`

</dd>
</dl>

<dl>
<dd>

**label:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**recipient_name:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**preferred_shipping:** `Option<Option<bool>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.patients.<a href="/src/api/resources/patients/client.rs">set_default_patient_address</a>(practice_id: String, patient_id: String, address_id: String) -> Result&lt;SetDefaultPatientAddressResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Changes delivery selection for future drafts, without changing patient clinical location or existing signed orders.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .patients
        .set_default_patient_address(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"pat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"addr_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**patient_id:** `String`

</dd>
</dl>

<dl>
<dd>

**address_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.patients.<a href="/src/api/resources/patients/client.rs">list_patients</a>(practice_id: String, ending_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, external_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, external_identity_source: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, external_identity_value: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, gender: Option&lt;Option&lt;Option&lt;ListPatientsRequestGender&gt;&gt;&gt;, last_order_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, last_order_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, program: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, query: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, sort: Option&lt;Option&lt;Option&lt;ListPatientsRequestSort&gt;&gt;&gt;, starting_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, states: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, status: Option&lt;Option&lt;Option&lt;ListPatientsRequestStatus&gt;&gt;&gt;) -> Result&lt;ListPatientsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists patients in one practice and mode. Use externalId for an exact match in the calling integration's namespace. Use externalIdentitySource with externalIdentityValue to search an explicit alias. Identity matching is case-sensitive after trimming whitespace. Other filters also apply.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .patients
        .list_patients(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &ListPatientsQueryRequest {
                ending_before: Some("pat_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                starting_after: Some("pat_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**ending_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**external_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**external_identity_source:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**external_identity_value:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**gender:** `Option<Option<ListPatientsRequestGender>>`

</dd>
</dl>

<dl>
<dd>

**last_order_after:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**last_order_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**program:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**query:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Option<ListPatientsRequestSort>>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**states:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<ListPatientsRequestStatus>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.patients.<a href="/src/api/resources/patients/client.rs">create_patient</a>(practice_id: String, request: CreatePatientRequest) -> Result&lt;CreatePatientResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a patient or resolves a matching externalId or external identity within this practice and mode. externalId belongs to the calling integration; externalIdentities holds aliases from other systems. Resolution preserves existing demographics; use PATCH to update them. Conflicting identifiers return 409. Email never merges patients. API keys require Idempotency-Key.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .patients
        .create_patient(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &CreatePatientRequest {
                date_of_birth: "dateOfBirth".to_string(),
                name: CreatePatientRequestName {
                    first: "first".to_string(),
                    last: "last".to_string(),
                    ..Default::default()
                },
                address: None,
                clinical_profile: None,
                email: None,
                external_id: None,
                external_identities: None,
                addresses: None,
                encounters: None,
                gender: None,
                location_id: None,
                metadata: None,
                medical_record_number: None,
                measurements: None,
                phone: None,
                programs: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**address:** `Option<Option<CreatePatientRequestAddress>>`

</dd>
</dl>

<dl>
<dd>

**clinical_profile:** `Option<Option<CreatePatientRequestClinicalProfile>>`

</dd>
</dl>

<dl>
<dd>

**date_of_birth:** `String`

</dd>
</dl>

<dl>
<dd>

**email:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**external_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**external_identities:** `Option<Option<Vec<CreatePatientRequestExternalIdentitiesItem>>>`

</dd>
</dl>

<dl>
<dd>

**addresses:** `Option<Option<Vec<CreatePatientRequestAddressesItem>>>`

</dd>
</dl>

<dl>
<dd>

**encounters:** `Option<Option<Vec<CreatePatientRequestEncountersItem>>>`

</dd>
</dl>

<dl>
<dd>

**gender:** `Option<Option<CreatePatientRequestGender>>`

</dd>
</dl>

<dl>
<dd>

**location_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>`

</dd>
</dl>

<dl>
<dd>

**medical_record_number:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**measurements:** `Option<Option<Vec<CreatePatientRequestMeasurementsItem>>>`

</dd>
</dl>

<dl>
<dd>

**name:** `CreatePatientRequestName`

</dd>
</dl>

<dl>
<dd>

**phone:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**programs:** `Option<Option<Vec<CreatePatientRequestProgramsItem>>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.patients.<a href="/src/api/resources/patients/client.rs">get_patient</a>(practice_id: String, patient_id: String) -> Result&lt;GetPatientResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns one patient in the authorized practice and mode.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .patients
        .get_patient(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"pat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**patient_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.patients.<a href="/src/api/resources/patients/client.rs">delete_patient</a>(practice_id: String, patient_id: String) -> Result&lt;DeletePatientResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires patients:write and Idempotency-Key for API keys. Permanently deletes a patient with no order history. Any order history returns 409; use Update patient with status archived instead. Available to practice keys and authorized platform keys. Reusing the same idempotency key returns the original deletion result.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .patients
        .delete_patient(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"pat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**patient_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.patients.<a href="/src/api/resources/patients/client.rs">update_patient</a>(practice_id: String, patient_id: String, request: UpdatePatientRequest) -> Result&lt;UpdatePatientResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a patient in the current practice and mode. Omitted fields remain unchanged; null clears an optional field. externalId updates the calling integration's identifier. externalIdentities replaces its explicit aliases. Identifiers cannot be reassigned from another patient. API keys require Idempotency-Key.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .patients
        .update_patient(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"pat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &UpdatePatientRequest {
                ..Default::default()
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**patient_id:** `String`

</dd>
</dl>

<dl>
<dd>

**address:** `Option<Option<UpdatePatientRequestAddress>>`

</dd>
</dl>

<dl>
<dd>

**clinical_profile:** `Option<Option<UpdatePatientRequestClinicalProfile>>`

</dd>
</dl>

<dl>
<dd>

**date_of_birth:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**email:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**external_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**external_identities:** `Option<Option<Vec<UpdatePatientRequestExternalIdentitiesItem>>>`

</dd>
</dl>

<dl>
<dd>

**addresses:** `Option<Option<Vec<UpdatePatientRequestAddressesItem>>>`

</dd>
</dl>

<dl>
<dd>

**encounters:** `Option<Option<Vec<UpdatePatientRequestEncountersItem>>>`

</dd>
</dl>

<dl>
<dd>

**gender:** `Option<Option<UpdatePatientRequestGender>>`

</dd>
</dl>

<dl>
<dd>

**location_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>`

</dd>
</dl>

<dl>
<dd>

**medical_record_number:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**measurements:** `Option<Option<Vec<UpdatePatientRequestMeasurementsItem>>>`

</dd>
</dl>

<dl>
<dd>

**name:** `Option<Option<UpdatePatientRequestName>>`

</dd>
</dl>

<dl>
<dd>

**programs:** `Option<Option<Vec<UpdatePatientRequestProgramsItem>>>`

</dd>
</dl>

<dl>
<dd>

**phone:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<UpdatePatientRequestStatus>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.patients.<a href="/src/api/resources/patients/client.rs">get_patient_allergies</a>(practice_id: String, patient_id: String) -> Result&lt;GetPatientAllergiesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the patient's structured allergy entries and review status. A not_reviewed status is not a no-known-allergies assertion and blocks clinical review and signing.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .patients
        .get_patient_allergies(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"pat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**patient_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.patients.<a href="/src/api/resources/patients/client.rs">replace_patient_allergies</a>(practice_id: String, patient_id: String, request: ReplacePatientAllergiesRequest) -> Result&lt;ReplacePatientAllergiesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Replaces the patient's structured allergy record. Sending no_known is the explicit no-known-allergies acknowledgement; recorded requires at least one entry. Idempotency-Key is required.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .patients
        .replace_patient_allergies(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &"pat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &ReplacePatientAllergiesRequest {
                allergies: vec![ReplacePatientAllergiesRequestAllergiesItem {
                    category: ReplacePatientAllergiesRequestAllergiesItemCategory::Drug,
                    code: None,
                    code_system: None,
                    reactions: vec![ReplacePatientAllergiesRequestAllergiesItemReactionsItem {
                        display: "display".to_string(),
                        ..Default::default()
                    }],
                    severity: None,
                    source: ReplacePatientAllergiesRequestAllergiesItemSource::Doctor,
                    substance: "substance".to_string(),
                    r#type: None,
                    verification_status:
                        ReplacePatientAllergiesRequestAllergiesItemVerificationStatus::Unconfirmed,
                }],
                review_status: ReplacePatientAllergiesRequestReviewStatus::NotReviewed,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**patient_id:** `String`

</dd>
</dl>

<dl>
<dd>

**allergies:** `Vec<ReplacePatientAllergiesRequestAllergiesItem>`

</dd>
</dl>

<dl>
<dd>

**review_status:** `ReplacePatientAllergiesRequestReviewStatus`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Practices
<details><summary><code>client.practices.<a href="/src/api/resources/practices/client.rs">list_practices</a>(search: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, ending_before: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, starting_after: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;ListPracticesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the practices that belong to the platform. The default Affinity-Version is 2026-09-28.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .practices
        .list_practices(
            &ListPracticesQueryRequest {
                ending_before: Some("prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                starting_after: Some("prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**search:** `Option<Option<String>>` — Case-insensitive search by practice name or external ID.

</dd>
</dl>

<dl>
<dd>

**ending_before:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**starting_after:** `Option<Option<String>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.practices.<a href="/src/api/resources/practices/client.rs">create_practice</a>(request: CreatePracticeRequest) -> Result&lt;CreatePracticeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a practice owned by the platform. Set liveEnabled to true to enable Live access at creation with an approved platform and a Live request. Defaults to false. Requires practices:write. Send Idempotency-Key when you retry the same request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .practices
        .create_practice(
            &CreatePracticeRequest {
                address: CreatePracticeRequestAddress {
                    city: "Los Angeles".to_string(),
                    country: Some("US".to_string()),
                    line1: "100 Practice Way".to_string(),
                    postal_code: "90001".to_string(),
                    state: "CA".to_string(),
                    ..Default::default()
                },
                attestations: CreatePracticeRequestAttestations {
                    authorized_practice_relationship: true,
                    authorized_phi_transfer: true,
                    minimum_necessary_phi: true,
                    provider_data_accuracy: true,
                    ..Default::default()
                },
                external_id: Some("practice_123".to_string()),
                legal_name: Some("Example Medical Group PLLC".to_string()),
                metadata: Some(HashMap::from([(
                    "key".to_string(),
                    serde_json::json!("value"),
                )])),
                name: "Example Medical Group".to_string(),
                prescribers: Some(vec![CreatePracticeRequestPrescribersItem {
                    credentials: Some("MD".to_string()),
                    license_states: vec!["CA".to_string()],
                    name: "Alex Morgan".to_string(),
                    npi: "1234567893".to_string(),
                    ..Default::default()
                }]),
                primary_contact: Some(CreatePracticeRequestPrimaryContact {
                    email: "operations@example-practice.com".to_string(),
                    name: "Jordan Lee".to_string(),
                    ..Default::default()
                }),
                support_email: Some("support@example-practice.com".to_string()),
                live_enabled: None,
                compliance_contact: None,
                support_phone: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**live_enabled:** `Option<bool>` — Enable Live access at creation. Requires an approved platform and a Live request. Defaults to false.

</dd>
</dl>

<dl>
<dd>

**address:** `CreatePracticeRequestAddress`

</dd>
</dl>

<dl>
<dd>

**attestations:** `CreatePracticeRequestAttestations`

</dd>
</dl>

<dl>
<dd>

**compliance_contact:** `Option<Option<CreatePracticeRequestComplianceContact>>`

</dd>
</dl>

<dl>
<dd>

**external_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**legal_name:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>`

</dd>
</dl>

<dl>
<dd>

**name:** `String`

</dd>
</dl>

<dl>
<dd>

**prescribers:** `Option<Option<Vec<CreatePracticeRequestPrescribersItem>>>`

</dd>
</dl>

<dl>
<dd>

**primary_contact:** `Option<Option<CreatePracticeRequestPrimaryContact>>`

</dd>
</dl>

<dl>
<dd>

**support_email:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**support_phone:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<Option<String>>` — Optional IANA timezone override. Omit to leave unchanged; null clears it. No timezone is inferred when creating a record.

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.practices.<a href="/src/api/resources/practices/client.rs">get_practice</a>(practice_id: String) -> Result&lt;GetPracticeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns one practice that belongs to the platform.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .practices
        .get_practice(&"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.practices.<a href="/src/api/resources/practices/client.rs">update_practice</a>(practice_id: String, request: UpdatePracticeRequest) -> Result&lt;UpdatePracticeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates one practice owned by the platform. Set liveEnabled to true or false to control Live access with an approved platform and a Live request. Affinity Admin decisions take precedence. Requires practices:write. Send Idempotency-Key when you retry the same request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .practices
        .update_practice(
            &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &UpdatePracticeRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**practice_id:** `String`

</dd>
</dl>

<dl>
<dd>

**live_enabled:** `Option<bool>` — Enable or disable Live access for an owned practice. Requires an approved platform and a Live request. Affinity Admin decisions take precedence.

</dd>
</dl>

<dl>
<dd>

**address:** `Option<Option<UpdatePracticeRequestAddress>>`

</dd>
</dl>

<dl>
<dd>

**attestations:** `Option<Option<UpdatePracticeRequestAttestations>>`

</dd>
</dl>

<dl>
<dd>

**compliance_contact:** `Option<Option<UpdatePracticeRequestComplianceContact>>`

</dd>
</dl>

<dl>
<dd>

**external_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**legal_name:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>`

</dd>
</dl>

<dl>
<dd>

**name:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**prescribers:** `Option<Option<Vec<UpdatePracticeRequestPrescribersItem>>>`

</dd>
</dl>

<dl>
<dd>

**primary_contact:** `Option<Option<UpdatePracticeRequestPrimaryContact>>`

</dd>
</dl>

<dl>
<dd>

**support_email:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**support_phone:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<Option<String>>` — Optional IANA timezone override. Omit to leave unchanged; null clears it. No timezone is inferred when creating a record.

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Platform Pricing
<details><summary><code>client.platform_pricing.<a href="/src/api/resources/platform_pricing/client.rs">platform_public_api_selling_prices_read_selling_price</a>(catalog_item_id: String, practice_id: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;) -> Result&lt;PlatformPublicApiSellingPricesReadSellingPriceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires selling_prices:read. Omit practiceId for the platform default, or supply a managed practice. A null amount inherits the next applicable price. Amounts use the catalog pricing basis, in USD cents. purchaseAmountCents is the platform's Affinity purchase price for that same basis. requiresReview indicates changed product pricing terms, not a below-purchase-price discount.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .platform_pricing
        .platform_public_api_selling_prices_read_selling_price(
            &"cat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &PlatformPublicAPISellingPricesReadSellingPriceQueryRequest {
                practice_id: Some("prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**catalog_item_id:** `String`

</dd>
</dl>

<dl>
<dd>

**practice_id:** `Option<Option<String>>`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.platform_pricing.<a href="/src/api/resources/platform_pricing/client.rs">platform_public_api_selling_prices_update_selling_price</a>(catalog_item_id: String, request: PlatformPublicApiSellingPricesUpdateSellingPriceRequest) -> Result&lt;PlatformPublicApiSellingPricesUpdateSellingPriceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires selling_prices:write. Sets a platform default or managed practice override in the current Test/Live mode. Send baseVersion from Read selling price. Null removes the override. Prices use the catalog pricing basis. Intentional discounts below purchaseAmountCents are allowed; compare these amounts to warn about selling below your Affinity purchase price. This does not change the platform's Affinity purchase price or collect practice payments.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use affinity_health_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .platform_pricing
        .platform_public_api_selling_prices_update_selling_price(
            &"cat_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
            &PlatformPublicAPISellingPricesUpdateSellingPriceRequest {
                base_version: 1,
                practice_id: None,
                amount_cents: None,
            },
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**catalog_item_id:** `String`

</dd>
</dl>

<dl>
<dd>

**practice_id:** `Option<Option<String>>`

</dd>
</dl>

<dl>
<dd>

**amount_cents:** `Option<i64>`

</dd>
</dl>

<dl>
<dd>

**base_version:** `i64`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>
