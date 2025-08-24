use std::env::{self, VarError};

use mcp_core::ToolError;
use serde::{Deserialize, Serialize};

// Define the request and response structures using Serde
#[derive(Debug, Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    choices: Vec<Choice>,
    // You can add other fields you might need here
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: String,
}

#[derive(Debug, Clone)]
pub struct AiSummarizer {
    pub api_key: String,
}

impl AiSummarizer {
    pub fn new() -> Result<Self, VarError> {
        Ok(Self {
            api_key: env::var("OPENROUTER_API_KEY")?,
        })
    }

    pub async fn summarize_docs_with_ai(
        &self,
        crate_name: &str,
        text: &str,
    ) -> Result<String, ToolError> {
        let response = reqwest::Client::new()
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            // OpenRouter requires this header to identify your application
            .header("HTTP-Referer", "cratedoc") // Replace with your URL or app name
            .header("X-Title", "Rust Docs MCP Server") // Replace with your app name
            .json(&serde_json::json!({
                "model": "tngtech/deepseek-r1t2-chimera:free",
                "messages": [
                    {
                        "role": "system",
                        "content": r"You are an expert technical writer for Rust documentation. Your task is to create a detailed yet structured overview of the provided crate documentation.

Please organize your summary into the following sections. You MUST use markdown formatting for readability:

1.  **Overview:** A 1-2 sentence description of the crate's purpose.
2.  **Key Features:** A bulleted list of its main features and capabilities.
3.  **Core API:** Describe the most important structs, enums, traits, and functions. Focus on what a user would need to know to get started.
4.  **Usage Example:** Provide a simple, practical code example showing how to use the crate in a common scenario.
5.  **Notable Considerations:** Mention any important details like safety warnings, async support, common dependencies, or configuration needs.

Be comprehensive but avoid simply listing everything. Curate the information for maximum usefulness to a Rust developer."
                    },
                    {
                        "role": "user",
                        "content": format!("Please analyze the following Rust crate documentation for the crate `{crate_name}` and provide a structured overview as requested:\n\n{}", text)
                    }
                ],
                "temperature": 0.2, // Low temperature for factual, deterministic output
                "max_tokens": 10_000,
            }))
            .send()
            .await
            .map_err(|e| {
                ToolError::ExecutionError(format!("Failed to summarize input: {e}"))
            })?;

        // 5. Check for errors and parse the response
        if !response.status().is_success() {
            let error_body = response.text().await.map_err(|e| {
                ToolError::ExecutionError(format!("failed to get error string: {e}"))
            })?;

            return Err(ToolError::ExecutionError(format!(
                "OpenRouter API error: {error_body}"
            )));
        }

        let api_response: OpenRouterResponse = response
            .json()
            .await
            .map_err(|e| ToolError::ExecutionError(format!("failed to get response json: {e}")))?;

        // 6. Extract the summary text
        if let Some(first_choice) = api_response.choices.first() {
            Ok(first_choice.message.content.trim().to_string())
        } else {
            Err(ToolError::ExecutionError(
                "No choices returned from OpenRouter API".to_owned(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn summarize() {
        let summary = AiSummarizer::new()
            .unwrap()
            .summarize_docs_with_ai("bevy", DOCS)
            .await
            .unwrap();

        assert!(!summary.is_empty());

        println!("{summary}");
    }

    pub const DOCS: &str = r###"[32m INFO[0m Testing tool: lookup_item
Executing lookup_item tool...

--- TOOL RESULT ---

Oklcha in bevy::color - Rustif(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-6b053e98.ttf.woff2,FiraSans-Italic-81dc35de.woff2,FiraSans-Regular-0fe48ade.woff2,FiraSans-MediumItalic-ccf7e434.woff2,FiraSans-Medium-e1aa3f0a.woff2,SourceCodePro-Regular-8badfe75.ttf.woff2,SourceCodePro-Semibold-aa29a496.ttf.woff2".split(",").map(f=\>`\<link rel="preload" as="font" type="font/woff2" crossorigin href="/-/rustdoc.static/${f}"\>`).join(""))\<link rel="stylesheet" href="/-/rustdoc.static/noscript-32bb7600.css"\> (function() { function applyTheme(theme) { if (theme) { document.documentElement.dataset.docsRsTheme = theme; } } window.addEventListener("storage", ev =\> { if (ev.key === "rustdoc-theme") { applyTheme(ev.newValue); } }); // see ./storage-change-detection.html for details window.addEventListener("message", ev =\> { if (ev.data && ev.data.storage && ev.data.storage.key === "rustdoc-theme") { applyTheme(ev.data.storage.value); } }); applyTheme(window.localStorage.getItem("rustdoc-theme"));
})();

[Docs.rs ](/)

 { "name": "bevy", "version": "0.16.1" }
* [bevy-0.16.1 ](#)

  * bevy 0.16.1
  * [ Permalink ](/bevy/0.16.1/bevy/color/struct.Oklcha.html)
  * [ Docs.rs crate page ](/crate/bevy/latest)
  * [MIT](https://spdx.org/licenses/MIT) OR [Apache-2.0](https://spdx.org/licenses/Apache-2.0)

  * Links
  * [ Homepage ](https://bevyengine.org)
  * [ Repository ](https://github.com/bevyengine/bevy)
  * [ crates.io ](https://crates.io/crates/bevy)
  * [ Source ](/crate/bevy/latest/source/)

  * Owners
  * [ cart ](https://crates.io/users/cart)
  * [ github:bevyengine:publish ](https://crates.io/teams/github:bevyengine:publish)

  * Dependencies
  * * [ bevy\_internal ^0.16.1 *normal* ](/bevy_internal/^0.16.1)
    * [ tracing ^0.1 *normal* *optional* ](/tracing/^0.1)
    * [ accesskit ^0.18 *dev* ](/accesskit/^0.18)
    * [ anyhow ^1 *dev* ](/anyhow/^1)
    * [ argh ^0.1.12 *dev* ](/argh/^0.1.12)
    * [ async-std ^1.13 *dev* ](/async-std/^1.13)
    * [ bevy\_asset ^0.16.1 *dev* ](/bevy_asset/^0.16.1)
    * [ bevy\_ecs ^0.16.1 *dev* ](/bevy_ecs/^0.16.1)
    * [ bevy\_gizmos ^0.16.1 *dev* ](/bevy_gizmos/^0.16.1)
    * [ bevy\_image ^0.16.1 *dev* ](/bevy_image/^0.16.1)
    * [ bevy\_reflect ^0.16.1 *dev* ](/bevy_reflect/^0.16.1)
    * [ bevy\_render ^0.16.1 *dev* ](/bevy_render/^0.16.1)
    * [ bevy\_state ^0.16.1 *dev* ](/bevy_state/^0.16.1)
    * [ bytemuck ^1.7 *dev* ](/bytemuck/^1.7)
    * [ crossbeam-channel ^0.5.0 *dev* ](/crossbeam-channel/^0.5.0)
    * [ event-listener ^5.3.0 *dev* ](/event-listener/^5.3.0)
    * [ flate2 ^1.0 *dev* ](/flate2/^1.0)
    * [ futures-lite ^2.0.1 *dev* ](/futures-lite/^2.0.1)
    * [ http-body-util ^0.1 *dev* ](/http-body-util/^0.1)
    * [ hyper ^1 *dev* ](/hyper/^1)
    * [ macro\_rules\_attribute ^0.2 *dev* ](/macro_rules_attribute/^0.2)
    * [ nonmax ^0.5 *dev* ](/nonmax/^0.5)
    * [ rand ^0.8.0 *dev* ](/rand/^0.8.0)
    * [ rand\_chacha ^0.3.1 *dev* ](/rand_chacha/^0.3.1)
    * [ ron ^0.8.0 *dev* ](/ron/^0.8.0)
    * [ serde ^1 *dev* ](/serde/^1)
    * [ serde\_json ^1.0.140 *dev* ](/serde_json/^1.0.140)
    * [ thiserror ^2.0 *dev* ](/thiserror/^2.0)
    * [ bevy\_dylib ^0.16.1 *normal* *optional* ](/bevy_dylib/^0.16.1)
    * [ smol ^2 *dev* ](/smol/^2)
    * [ smol-hyper ^0.1 *dev* ](/smol-hyper/^0.1)
    * [ smol-macros ^0.1 *dev* ](/smol-macros/^0.1)
    * [ ureq ^3.0.8 *dev* ](/ureq/^3.0.8)
    * [ wasm-bindgen ^0.2 *dev* ](/wasm-bindgen/^0.2)
    * [ web-sys ^0.3 *dev* ](/web-sys/^0.3)

  * Versions
  *

  * [**100%** of the crate is documented ](/crate/bevy/latest)

* [Platform ](#)
  * [x86\_64-unknown-linux-gnu](/crate/bevy/latest/target-redirect/x86_64-unknown-linux-gnu/bevy/color/struct.Oklcha.html)

* [Feature flags ](/crate/bevy/latest/features)

* [docs.rs](#)
  * [ About docs.rs](/about)
  * [ Badges](/about/badges)
  * [ Builds](/about/builds)
  * [ Metadata](/about/metadata)
  * [ Shorthand URLs](/about/redirections)
  * [ Download](/about/download)
  * [ Rustdoc JSON](/about/rustdoc-json)
  * [ Build queue](/releases/queue)
  * [ Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

* [Rust](#)
  * [Rust website](https://www.rust-lang.org/)
  * [The Book](https://doc.rust-lang.org/book/)
  * [Standard Library API Reference](https://doc.rust-lang.org/std/)
  * [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  * [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  * [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

<iframe src="/-/storage-change-detection.html" width="0" height="0" style="display: none"></iframe>

[![](https://bevyengine.org/assets/icon.png)](../../bevy/index.html)

[![logo](https://bevyengine.org/assets/icon.png)](../../bevy/index.html)

[bevy](../../bevy/index.html)0.16.1
----------

[Oklcha](#)
----------

### [Sections](#) ###

* [Conversion](#conversion)

### [Fields](#fields) ###

* [alpha](#structfield.alpha)
* [chroma](#structfield.chroma)
* [hue](#structfield.hue)
* [lightness](#structfield.lightness)

### [Methods](#implementations) ###

* [lch](#method.lch)
* [new](#method.new)
* [sequential\_dispersed](#method.sequential_dispersed)
* [with\_chroma](#method.with_chroma)
* [with\_lightness](#method.with_lightness)

### [Trait Implementations](#trait-implementations) ###

* [Alpha](#impl-Alpha-for-Oklcha)
* [Clone](#impl-Clone-for-Oklcha)
* [ColorToComponents](#impl-ColorToComponents-for-Oklcha)
* [Copy](#impl-Copy-for-Oklcha)
* [Debug](#impl-Debug-for-Oklcha)
* [Default](#impl-Default-for-Oklcha)
* [Deserialize\<'de\>](#impl-Deserialize%3C'de%3E-for-Oklcha)
* [EuclideanDistance](#impl-EuclideanDistance-for-Oklcha)
* [From\<Color\>](#impl-From%3CColor%3E-for-Oklcha)
* [From\<Hsla\>](#impl-From%3CHsla%3E-for-Oklcha)
* [From\<Hsva\>](#impl-From%3CHsva%3E-for-Oklcha)
* [From\<Hwba\>](#impl-From%3CHwba%3E-for-Oklcha)
* [From\<Laba\>](#impl-From%3CLaba%3E-for-Oklcha)
* [From\<Lcha\>](#impl-From%3CLcha%3E-for-Oklcha)
* [From\<LinearRgba\>](#impl-From%3CLinearRgba%3E-for-Oklcha)
* [From\<Oklaba\>](#impl-From%3COklaba%3E-for-Oklcha)
* [From\<Oklcha\>](#impl-From%3COklcha%3E-for-Color)
* [From\<Oklcha\>](#impl-From%3COklcha%3E-for-Hsla)
* [From\<Oklcha\>](#impl-From%3COklcha%3E-for-Hsva)
* [From\<Oklcha\>](#impl-From%3COklcha%3E-for-Hwba)
* [From\<Oklcha\>](#impl-From%3COklcha%3E-for-Laba)
* [From\<Oklcha\>](#impl-From%3COklcha%3E-for-Lcha)
* [From\<Oklcha\>](#impl-From%3COklcha%3E-for-LinearRgba)
* [From\<Oklcha\>](#impl-From%3COklcha%3E-for-Oklaba)
* [From\<Oklcha\>](#impl-From%3COklcha%3E-for-Srgba)
* [From\<Oklcha\>](#impl-From%3COklcha%3E-for-Xyza)
* [From\<Srgba\>](#impl-From%3CSrgba%3E-for-Oklcha)
* [From\<Xyza\>](#impl-From%3CXyza%3E-for-Oklcha)
* [FromArg](#impl-FromArg-for-%26Oklcha)
* [FromArg](#impl-FromArg-for-%26mut+Oklcha)
* [FromArg](#impl-FromArg-for-Oklcha)
* [FromReflect](#impl-FromReflect-for-Oklcha)
* [GetOwnership](#impl-GetOwnership-for-%26Oklcha)
* [GetOwnership](#impl-GetOwnership-for-%26mut+Oklcha)
* [GetOwnership](#impl-GetOwnership-for-Oklcha)
* [GetTypeRegistration](#impl-GetTypeRegistration-for-Oklcha)
* [Gray](#impl-Gray-for-Oklcha)
* [Hue](#impl-Hue-for-Oklcha)
* [IntoReturn](#impl-IntoReturn-for-%26Oklcha)
* [IntoReturn](#impl-IntoReturn-for-%26mut+Oklcha)
* [IntoReturn](#impl-IntoReturn-for-Oklcha)
* [Luminance](#impl-Luminance-for-Oklcha)
* [Mix](#impl-Mix-for-Oklcha)
* [PartialEq](#impl-PartialEq-for-Oklcha)
* [PartialReflect](#impl-PartialReflect-for-Oklcha)
* [Reflect](#impl-Reflect-for-Oklcha)
* [Serialize](#impl-Serialize-for-Oklcha)
* [Struct](#impl-Struct-for-Oklcha)
* [StructuralPartialEq](#impl-StructuralPartialEq-for-Oklcha)
* [TypePath](#impl-TypePath-for-Oklcha)
* [Typed](#impl-Typed-for-Oklcha)

### [Auto Trait Implementations](#synthetic-implementations) ###

* [Freeze](#impl-Freeze-for-Oklcha)
* [RefUnwindSafe](#impl-RefUnwindSafe-for-Oklcha)
* [Send](#impl-Send-for-Oklcha)
* [Sync](#impl-Sync-for-Oklcha)
* [Unpin](#impl-Unpin-for-Oklcha)
* [UnwindSafe](#impl-UnwindSafe-for-Oklcha)

### [Blanket Implementations](#blanket-implementations) ###

* [Any](#impl-Any-for-T)
* [AsBindGroupShaderType\<U\>](#impl-AsBindGroupShaderType%3CU%3E-for-T)
* [Borrow\<T\>](#impl-Borrow%3CT%3E-for-T)
* [BorrowMut\<T\>](#impl-BorrowMut%3CT%3E-for-T)
* [CloneToUninit](#impl-CloneToUninit-for-T)
* [ConditionalSend](#impl-ConditionalSend-for-T)
* [Conv](#impl-Conv-for-T)
* [DeserializeOwned](#impl-DeserializeOwned-for-T)
* [Downcast](#impl-Downcast-for-T)
* [Downcast](#impl-Downcast-for-T-1)
* [Downcast\<T\>](#impl-Downcast%3CT%3E-for-T)
* [DowncastSend](#impl-DowncastSend-for-T)
* [DowncastSync](#impl-DowncastSync-for-T)
* [Duplex\<S\>](#impl-Duplex%3CS%3E-for-T)
* [DynamicTypePath](#impl-DynamicTypePath-for-T)
* [DynamicTyped](#impl-DynamicTyped-for-T)
* [FmtForward](#impl-FmtForward-for-T)
* [From\<T\>](#impl-From%3CT%3E-for-T)
* [FromSample\<S\>](#impl-FromSample%3CS%3E-for-S)
* [FromWorld](#impl-FromWorld-for-T)
* [GetField](#impl-GetField-for-S)
* [GetPath](#impl-GetPath-for-T)
* [Instrument](#impl-Instrument-for-T)
* [Into\<U\>](#impl-Into%3CU%3E-for-T)
* [IntoEither](#impl-IntoEither-for-T)
* [IntoSample\<T\>](#impl-IntoSample%3CT%3E-for-F)
* [NoneValue](#impl-NoneValue-for-T)
* [Pipe](#impl-Pipe-for-T)
* [Pointable](#impl-Pointable-for-T)
* [ReadPrimitive\<R\>](#impl-ReadPrimitive%3CR%3E-for-P)
* [Reflectable](#impl-Reflectable-for-T)
* [Serialize](#impl-Serialize-for-T)
* [Settings](#impl-Settings-for-T)
* [Tap](#impl-Tap-for-T)
* [ToOwned](#impl-ToOwned-for-T)
* [ToSample\<U\>](#impl-ToSample%3CU%3E-for-T)
* [TryConv](#impl-TryConv-for-T)
* [TryFrom\<U\>](#impl-TryFrom%3CU%3E-for-T)
* [TryInto\<U\>](#impl-TryInto%3CU%3E-for-T)
* [TypeData](#impl-TypeData-for-T)
* [Upcast\<T\>](#impl-Upcast%3CT%3E-for-T)
* [VZip\<V\>](#impl-VZip%3CV%3E-for-T)
* [WasmNotSend](#impl-WasmNotSend-for-T)
* [WasmNotSendSync](#impl-WasmNotSendSync-for-T)
* [WasmNotSync](#impl-WasmNotSync-for-T)
* [WithSubscriber](#impl-WithSubscriber-for-T)

[In bevy::color](index.html)
----------

[bevy](../index.html)::[color](index.html)

Struct OklchaCopy item path
==========

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#25)

```
pub struct Oklcha {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
    pub alpha: f32,
}
```

<details class="toggle top-doc" open=""><summary class="hideme">Expand description</summary>

Color in Oklch color space, with alpha

[§](#conversion)Conversion
----------

Conversion between the various color spaces is achieved using Rust’s native [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) trait. Because certain color spaces are defined by their transformation to and from another space, these [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) implementations reflect that set of definitions.

```
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

For example, the [sRGB](../prelude/struct.Srgba.html) space is defined by its relationship with [Linear RGB](../prelude/struct.LinearRgba.html), and [HWB](../prelude/struct.Hwba.html) by its with [sRGB](../prelude/struct.Srgba.html). As such, it is the responsibility of [sRGB](../prelude/struct.Srgba.html) to provide [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) implementations for [Linear RGB](../prelude/struct.LinearRgba.html), and [HWB](../prelude/struct.Hwba.html) for [sRGB](../prelude/struct.Srgba.html). To then provide conversion between [Linear RGB](../prelude/struct.LinearRgba.html) and [HWB](../prelude/struct.Hwba.html) directly, [HWB](../prelude/struct.Hwba.html) is responsible for implementing these conversions, delegating to [sRGB](../prelude/struct.Srgba.html) as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

@import url("https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css");'#graph-div{font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:16px;fill:#ccc;}#graph-div .error-icon{fill:#a44141;}#graph-div .error-text{fill:#ddd;stroke:#ddd;}#graph-div .edge-thickness-normal{stroke-width:2px;}#graph-div .edge-thickness-thick{stroke-width:3.5px;}#graph-div .edge-pattern-solid{stroke-dasharray:0;}#graph-div .edge-pattern-dashed{stroke-dasharray:3;}#graph-div .edge-pattern-dotted{stroke-dasharray:2;}#graph-div .marker{fill:lightgrey;stroke:lightgrey;}#graph-div .marker.cross{stroke:lightgrey;}#graph-div svg{font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:16px;}#graph-div .label{font-family:"trebuchet ms",verdana,arial,sans-serif;color:#ccc;}#graph-div .cluster-label text{fill:#F9FFFE;}#graph-div .cluster-label span,#graph-div p{color:#F9FFFE;}#graph-div .label text,#graph-div span,#graph-div p{fill:#ccc;color:#ccc;}#graph-div .node rect,#graph-div .node circle,#graph-div .node ellipse,#graph-div .node polygon,#graph-div .node path{fill:#1f2020;stroke:#81B1DB;stroke-width:1px;}#graph-div .flowchart-label text{text-anchor:middle;}#graph-div .node .label{text-align:center;}#graph-div .node.clickable{cursor:pointer;}#graph-div .arrowheadPath{fill:lightgrey;}#graph-div .edgePath .path{stroke:lightgrey;stroke-width:2.0px;}#graph-div .flowchart-link{stroke:lightgrey;fill:none;}#graph-div .edgeLabel{background-color:hsl(0, 0%, 34.4117647059%);text-align:center;}#graph-div .edgeLabel rect{opacity:0.5;background-color:hsl(0, 0%, 34.4117647059%);fill:hsl(0, 0%, 34.4117647059%);}#graph-div .labelBkg{background-color:rgba(87.75, 87.75, 87.75, 0.5);}#graph-div .cluster rect{fill:hsl(180, 1.5873015873%, 28.3529411765%);stroke:rgba(255, 255, 255, 0.25);stroke-width:1px;}#graph-div .cluster text{fill:#F9FFFE;}#graph-div .cluster span,#graph-div p{color:#F9FFFE;}#graph-div div.mermaidTooltip{position:absolute;text-align:center;max-width:200px;padding:2px;font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:12px;background:hsl(20, 1.5873015873%, 12.3529411765%);border:1px solid rgba(255, 255, 255, 0.25);border-radius:2px;pointer-events:none;z-index:100;}#graph-div .flowchartTitleText{text-anchor:middle;font-size:18px;fill:#ccc;}#graph-div .label foreignObject{overflow:visible;}#graph-div :root{--mermaid-font-family:"trebuchet ms",verdana,arial,sans-serif;}

[Conversion](https://bottosson.github.io/posts/oklab/#converting-from-linear-srgb-to-oklab)

[Conversion](https://bottosson.github.io/posts/oklab/#the-oklab-color-space)

[Conversion](http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html)

[Conversion](http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_Lab.html)

[Conversion](http://www.brucelindbloom.com/index.html?Eqn_Lab_to_LCH.html)

[Conversion](https://en.wikipedia.org/wiki/SRGB#From_sRGB_to_CIE_XYZ)

[Conversion](http://alvyray.com/Papers/CG/HWB_JGTv208.pdf)

[Conversion](http://alvyray.com/Papers/CG/HWB_JGTv208.pdf)

[Conversion](https://en.wikipedia.org/wiki/HSL_and_HSV#Interconversion)

[Linear  
sRGB](https://en.wikipedia.org/wiki/Rgb)

[Oklab](https://oklch.com/)

[Oklch](https://oklch.com/)

[XYZ](https://en.wikipedia.org/wiki/XYZ_color)

[Lab](https://en.wikipedia.org/wiki/Lab_color)

[Lch](https://en.wikipedia.org/wiki/CIELAB_color_space#Cylindrical_model)

[sRGB](https://en.wikipedia.org/wiki/Srgb)

[HWB](https://en.wikipedia.org/wiki/HWB_color_model)

[HSV](https://en.wikipedia.org/wiki/HSL_and_HSV)

[HSL](https://en.wikipedia.org/wiki/HSL_and_HSV)

GPU

</details>

Fields[§](#fields)
----------

[§](#structfield.lightness)`lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)`

The ‘lightness’ channel. [0.0, 1.0]

[§](#structfield.chroma)`chroma: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)`

The ‘chroma’ channel. [0.0, 1.0]

[§](#structfield.hue)`hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)`

The ‘hue’ channel. [0.0, 360.0]

[§](#structfield.alpha)`alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)`

The alpha channel. [0.0, 1.0]

Implementations[§](#implementations)
----------

<details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#38)[§](#impl-Oklcha)

### impl [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#47)

#### pub const fn [new](#method.new)(lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), chroma: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Construct a new [`Oklcha`](../prelude/struct.Oklcha.html) color from components.

##### [§](#arguments)Arguments #####

* `lightness` - Lightness channel. [0.0, 1.0]
* `chroma` - Chroma channel. [0.0, 1.0]
* `hue` - Hue channel. [0.0, 360.0]
* `alpha` - Alpha channel. [0.0, 1.0]

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#64)

#### pub const fn [lch](#method.lch)(lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), chroma: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Construct a new [`Oklcha`](../prelude/struct.Oklcha.html) color from (l, c, h) components, with the default alpha (1.0).

##### [§](#arguments-1)Arguments #####

* `lightness` - Lightness channel. [0.0, 1.0]
* `chroma` - Chroma channel. [0.0, 1.0]
* `hue` - Hue channel. [0.0, 360.0]
* `alpha` - Alpha channel. [0.0, 1.0]

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#69)

#### pub const fn [with\_lightness](#method.with_lightness)(self, lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Return a copy of this color with the ‘lightness’ channel set to the given value.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#74)

#### pub const fn [with\_chroma](#method.with_chroma)(self, chroma: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Return a copy of this color with the ‘chroma’ channel set to the given value.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#95)

#### pub fn [sequential\_dispersed](#method.sequential_dispersed)(index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Generate a deterministic but [quasi-randomly distributed](https://en.wikipedia.org/wiki/Low-discrepancy_sequence)color from a provided `index`.

This can be helpful for generating debug colors.

##### [§](#examples)Examples #####

```
// Unique color for an entity
// let entity_index = entity.index();
let color = Oklcha::sequential_dispersed(entity_index);

// Palette with 5 distinct hues
let palette = (0..5).map(Oklcha::sequential_dispersed).collect::<Vec<_>>();
```

</details>

</details>

Trait Implementations[§](#trait-implementations)
----------

<details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#132)[§](#impl-Alpha-for-Oklcha)

### impl [Alpha](../prelude/trait.Alpha.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#134)[§](#method.with_alpha)

#### fn [with\_alpha](../prelude/trait.Alpha.html#tymethod.with_alpha)(&self, alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Return a new version of this color with the given alpha value.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#139)[§](#method.alpha)

#### fn [alpha](../prelude/trait.Alpha.html#tymethod.alpha)(&self) -\> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html) ####

</summary>

Return a the alpha component of this color.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#144)[§](#method.set_alpha)

#### fn [set\_alpha](../prelude/trait.Alpha.html#tymethod.set_alpha)(&mut self, alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) ####

</summary>

Sets the alpha component of this color.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#70)[§](#method.is_fully_transparent)

#### fn [is\_fully\_transparent](../prelude/trait.Alpha.html#method.is_fully_transparent)(&self) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) ####

</summary>

Is the alpha component of this color less than or equal to 0.0?

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#75)[§](#method.is_fully_opaque)

#### fn [is\_fully\_opaque](../prelude/trait.Alpha.html#method.is_fully_opaque)(&self) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) ####

</summary>

Is the alpha component of this color greater than or equal to 1.0?

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#14)[§](#impl-Clone-for-Oklcha)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#14)[§](#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

</details><details class="toggle method-toggle" open=""><summary>

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174)[§](#method.clone_from)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self) ####

</summary>

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#204)[§](#impl-ColorToComponents-for-Oklcha)

### impl [ColorToComponents](../prelude/trait.ColorToComponents.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#205)[§](#method.to_f32_array)

#### fn [to\_f32\_array](../prelude/trait.ColorToComponents.html#tymethod.to_f32_array)(self) -\> [[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)] ####

</summary>

Convert to an f32 array

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#209)[§](#method.to_f32_array_no_alpha)

#### fn [to\_f32\_array\_no\_alpha](../prelude/trait.ColorToComponents.html#tymethod.to_f32_array_no_alpha)(self) -\> [[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)] ####

</summary>

Convert to an f32 array without the alpha value

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#213)[§](#method.to_vec4)

#### fn [to\_vec4](../prelude/trait.ColorToComponents.html#tymethod.to_vec4)(self) -\> [Vec4](../prelude/struct.Vec4.html) ####

</summary>

Convert to a Vec4

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#217)[§](#method.to_vec3)

#### fn [to\_vec3](../prelude/trait.ColorToComponents.html#tymethod.to_vec3)(self) -\> [Vec3](../prelude/struct.Vec3.html) ####

</summary>

Convert to a Vec3

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#221)[§](#method.from_f32_array)

#### fn [from\_f32\_array](../prelude/trait.ColorToComponents.html#tymethod.from_f32_array)(color: [[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)]) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Convert from an f32 array

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#230)[§](#method.from_f32_array_no_alpha)

#### fn [from\_f32\_array\_no\_alpha](../prelude/trait.ColorToComponents.html#tymethod.from_f32_array_no_alpha)(color: [[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)]) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Convert from an f32 array without the alpha value

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#239)[§](#method.from_vec4)

#### fn [from\_vec4](../prelude/trait.ColorToComponents.html#tymethod.from_vec4)(color: [Vec4](../prelude/struct.Vec4.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Convert from a Vec4

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#248)[§](#method.from_vec3)

#### fn [from\_vec3](../prelude/trait.ColorToComponents.html#tymethod.from_vec3)(color: [Vec3](../prelude/struct.Vec3.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Convert from a Vec3

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#14)[§](#impl-Debug-for-Oklcha)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#14)[§](#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html)\<'\_\>) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html)\> ####

</summary>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#108)[§](#impl-Default-for-Oklcha)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#109)[§](#method.default)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#20)[§](#impl-Deserialize%3C'de%3E-for-Oklcha)

### impl\<'de\> [Deserialize](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html)\<'de\> for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#20)[§](#method.deserialize)

#### fn [deserialize](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)\<\_\_D\>( \_\_deserializer: \_\_D,
) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<[Oklcha](../prelude/struct.Oklcha.html), \<\_\_D as [Deserializer](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html)\<'de\>\>::[Error](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error)\>

where \_\_D: [Deserializer](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html)\<'de\>,

 ####

</summary>

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#195)[§](#impl-EuclideanDistance-for-Oklcha)

### impl [EuclideanDistance](color_difference/trait.EuclideanDistance.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#197)[§](#method.distance_squared)

#### fn [distance\_squared](color_difference/trait.EuclideanDistance.html#tymethod.distance_squared)(&self, other: &[Oklcha](../prelude/struct.Oklcha.html)) -\> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html) ####

</summary>

Distance squared from `self` to `other`.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/color_difference.rs.html#9)[§](#method.distance)

#### fn [distance](color_difference/trait.EuclideanDistance.html#method.distance)(&self, other: &Self) -\> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html) ####

</summary>

Distance from `self` to `other`.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#666)[§](#impl-From%3CColor%3E-for-Oklcha)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Color](../prelude/enum.Color.html)\> for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#667)[§](#method.from-1)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Color](../prelude/enum.Color.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#296)[§](#impl-From%3CHsla%3E-for-Oklcha)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Hsla](../prelude/struct.Hsla.html)\> for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#297)[§](#method.from-4)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Hsla](../prelude/struct.Hsla.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#308)[§](#impl-From%3CHsva%3E-for-Oklcha)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Hsva](../prelude/struct.Hsva.html)\> for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#309)[§](#method.from-6)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Hsva](../prelude/struct.Hsva.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#320)[§](#impl-From%3CHwba%3E-for-Oklcha)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Hwba](../prelude/struct.Hwba.html)\> for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#321)[§](#method.from-8)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Hwba](../prelude/struct.Hwba.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#332)[§](#impl-From%3CLaba%3E-for-Oklcha)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Laba](../prelude/struct.Laba.html)\> for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#333)[§](#method.from-10)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Laba](../prelude/struct.Laba.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#344)[§](#impl-From%3CLcha%3E-for-Oklcha)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Lcha](../prelude/struct.Lcha.html)\> for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#345)[§](#method.from-12)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Lcha](../prelude/struct.Lcha.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#356)[§](#impl-From%3CLinearRgba%3E-for-Oklcha)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[LinearRgba](../prelude/struct.LinearRgba.html)\> for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#357)[§](#method.from-14)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [LinearRgba](../prelude/struct.LinearRgba.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#258)[§](#impl-From%3COklaba%3E-for-Oklcha)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Oklaba](../prelude/struct.Oklaba.html)\> for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#259-266)[§](#method.from-2)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: [Oklaba](../prelude/struct.Oklaba.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#44)[§](#impl-From%3COklcha%3E-for-Color)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Oklcha](../prelude/struct.Oklcha.html)\> for [Color](../prelude/enum.Color.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#44)[§](#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Oklcha](../prelude/struct.Oklcha.html)) -\> [Color](../prelude/enum.Color.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#302)[§](#impl-From%3COklcha%3E-for-Hsla)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Oklcha](../prelude/struct.Oklcha.html)\> for [Hsla](../prelude/struct.Hsla.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#303)[§](#method.from-5)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Oklcha](../prelude/struct.Oklcha.html)) -\> [Hsla](../prelude/struct.Hsla.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#314)[§](#impl-From%3COklcha%3E-for-Hsva)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Oklcha](../prelude/struct.Oklcha.html)\> for [Hsva](../prelude/struct.Hsva.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#315)[§](#method.from-7)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Oklcha](../prelude/struct.Oklcha.html)) -\> [Hsva](../prelude/struct.Hsva.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#326)[§](#impl-From%3COklcha%3E-for-Hwba)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Oklcha](../prelude/struct.Oklcha.html)\> for [Hwba](../prelude/struct.Hwba.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#327)[§](#method.from-9)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Oklcha](../prelude/struct.Oklcha.html)) -\> [Hwba](../prelude/struct.Hwba.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#338)[§](#impl-From%3COklcha%3E-for-Laba)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Oklcha](../prelude/struct.Oklcha.html)\> for [Laba](../prelude/struct.Laba.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#339)[§](#method.from-11)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Oklcha](../prelude/struct.Oklcha.html)) -\> [Laba](../prelude/struct.Laba.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#350)[§](#impl-From%3COklcha%3E-for-Lcha)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Oklcha](../prelude/struct.Oklcha.html)\> for [Lcha](../prelude/struct.Lcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#351)[§](#method.from-13)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Oklcha](../prelude/struct.Oklcha.html)) -\> [Lcha](../prelude/struct.Lcha.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#362)[§](#impl-From%3COklcha%3E-for-LinearRgba)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Oklcha](../prelude/struct.Oklcha.html)\> for [LinearRgba](../prelude/struct.LinearRgba.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#363)[§](#method.from-15)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Oklcha](../prelude/struct.Oklcha.html)) -\> [LinearRgba](../prelude/struct.LinearRgba.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#276)[§](#impl-From%3COklcha%3E-for-Oklaba)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Oklcha](../prelude/struct.Oklcha.html)\> for [Oklaba](../prelude/struct.Oklaba.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#277-284)[§](#method.from-3)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: [Oklcha](../prelude/struct.Oklcha.html)) -\> [Oklaba](../prelude/struct.Oklaba.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#374)[§](#impl-From%3COklcha%3E-for-Srgba)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Oklcha](../prelude/struct.Oklcha.html)\> for [Srgba](../prelude/struct.Srgba.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#375)[§](#method.from-17)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Oklcha](../prelude/struct.Oklcha.html)) -\> [Srgba](../prelude/struct.Srgba.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#386)[§](#impl-From%3COklcha%3E-for-Xyza)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Oklcha](../prelude/struct.Oklcha.html)\> for [Xyza](../prelude/struct.Xyza.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#387)[§](#method.from-19)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Oklcha](../prelude/struct.Oklcha.html)) -\> [Xyza](../prelude/struct.Xyza.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#368)[§](#impl-From%3CSrgba%3E-for-Oklcha)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Srgba](../prelude/struct.Srgba.html)\> for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#369)[§](#method.from-16)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Srgba](../prelude/struct.Srgba.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#380)[§](#impl-From%3CXyza%3E-for-Oklcha)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<[Xyza](../prelude/struct.Xyza.html)\> for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#381)[§](#method.from-18)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Xyza](../prelude/struct.Xyza.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Converts to this type from the input type.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-FromArg-for-%26Oklcha)

### impl [FromArg](../reflect/func/args/trait.FromArg.html) for &'static [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#associatedtype.This-1)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)\<'from\_arg\> = &'from\_arg [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.from_arg-1)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)( arg: [Arg](../reflect/func/args/struct.Arg.html)\<'\_\>,
) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<\<&'static [Oklcha](../prelude/struct.Oklcha.html) as [FromArg](../reflect/func/args/trait.FromArg.html)\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This)\<'\_\>, [ArgError](../reflect/func/enum.ArgError.html)\> ####

</summary>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-FromArg-for-%26mut+Oklcha)

### impl [FromArg](../reflect/func/args/trait.FromArg.html) for &'static mut [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#associatedtype.This-2)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)\<'from\_arg\> = &'from\_arg mut [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.from_arg-2)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)( arg: [Arg](../reflect/func/args/struct.Arg.html)\<'\_\>,
) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<\<&'static mut [Oklcha](../prelude/struct.Oklcha.html) as [FromArg](../reflect/func/args/trait.FromArg.html)\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This)\<'\_\>, [ArgError](../reflect/func/enum.ArgError.html)\> ####

</summary>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-FromArg-for-Oklcha)

### impl [FromArg](../reflect/func/args/trait.FromArg.html) for [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#associatedtype.This)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)\<'from\_arg\> = [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.from_arg)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html)\<'\_\>) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<\<[Oklcha](../prelude/struct.Oklcha.html) as [FromArg](../reflect/func/args/trait.FromArg.html)\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This)\<'\_\>, [ArgError](../reflect/func/enum.ArgError.html)\> ####

</summary>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-FromReflect-for-Oklcha)

### impl [FromReflect](../prelude/trait.FromReflect.html) for [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.from_reflect)

#### fn [from\_reflect](../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html) + 'static)) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<[Oklcha](../prelude/struct.Oklcha.html)\> ####

</summary>

Constructs a concrete instance of `Self` from a reflected value.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)[§](#method.take_from_reflect)

#### fn [take\_from\_reflect](../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../prelude/struct.Box.html)\<dyn [PartialReflect](../prelude/trait.PartialReflect.html)\>,
) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<Self, [Box](../prelude/struct.Box.html)\<dyn [PartialReflect](../prelude/trait.PartialReflect.html)\>\> ####

</summary>

Attempts to downcast the given value to `Self` using,
constructing the value using [`from_reflect`](../prelude/trait.FromReflect.html#tymethod.from_reflect) if that fails. [Read more](../prelude/trait.FromReflect.html#method.take_from_reflect)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-GetOwnership-for-%26Oklcha)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html) for &[Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.ownership-1)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#tymethod.ownership)() -\> [Ownership](../reflect/func/args/enum.Ownership.html) ####

</summary>

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html).

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-GetOwnership-for-%26mut+Oklcha)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html) for &mut [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.ownership-2)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#tymethod.ownership)() -\> [Ownership](../reflect/func/args/enum.Ownership.html) ####

</summary>

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html).

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-GetOwnership-for-Oklcha)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html) for [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.ownership)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#tymethod.ownership)() -\> [Ownership](../reflect/func/args/enum.Ownership.html) ####

</summary>

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html).

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-GetTypeRegistration-for-Oklcha)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html) for [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.get_type_registration)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -\> [TypeRegistration](../reflect/struct.TypeRegistration.html) ####

</summary>

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html) for this type.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.register_type_dependencies)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html)) ####

</summary>

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#127)[§](#impl-Gray-for-Oklcha)

### impl [Gray](../prelude/trait.Gray.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#128)[§](#associatedconstant.BLACK)

#### const [BLACK](../prelude/trait.Gray.html#associatedconstant.BLACK): [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

A pure black color.

</details><details class="toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#129)[§](#associatedconstant.WHITE)

#### const [WHITE](../prelude/trait.Gray.html#associatedconstant.WHITE): [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

A pure white color.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#53)[§](#method.gray)

#### fn [gray](../prelude/trait.Gray.html#method.gray)(lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -\> Self ####

</summary>

Returns a grey color with the provided lightness from (0.0 - 1.0). 0 is black, 1 is white.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#149)[§](#impl-Hue-for-Oklcha)

### impl [Hue](../prelude/trait.Hue.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#151)[§](#method.with_hue)

#### fn [with\_hue](../prelude/trait.Hue.html#tymethod.with_hue)(&self, hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Return a new version of this color with the hue channel set to the given value.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#156)[§](#method.hue)

#### fn [hue](../prelude/trait.Hue.html#tymethod.hue)(&self) -\> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html) ####

</summary>

Return the hue of this color [0.0, 360.0].

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#161)[§](#method.set_hue)

#### fn [set\_hue](../prelude/trait.Hue.html#tymethod.set_hue)(&mut self, hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) ####

</summary>

Sets the hue of this color.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#92)[§](#method.rotate_hue)

#### fn [rotate\_hue](../prelude/trait.Hue.html#method.rotate_hue)(&self, degrees: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -\> Self ####

</summary>

Return a new version of this color with the hue channel rotated by the given degrees.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-IntoReturn-for-%26Oklcha)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html) for &[Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.into_return-1)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)\<'into\_return\>(self) -\> [Return](../reflect/func/enum.Return.html)\<'into\_return\>

where &[Oklcha](../prelude/struct.Oklcha.html): 'into\_return,

 ####

</summary>

Converts [`Self`](../reflect/func/trait.IntoReturn.html) into a [`Return`](../reflect/func/enum.Return.html) value.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-IntoReturn-for-%26mut+Oklcha)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html) for &mut [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.into_return-2)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)\<'into\_return\>(self) -\> [Return](../reflect/func/enum.Return.html)\<'into\_return\>

where &mut [Oklcha](../prelude/struct.Oklcha.html): 'into\_return,

 ####

</summary>

Converts [`Self`](../reflect/func/trait.IntoReturn.html) into a [`Return`](../reflect/func/enum.Return.html) value.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-IntoReturn-for-Oklcha)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html) for [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.into_return)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)\<'into\_return\>(self) -\> [Return](../reflect/func/enum.Return.html)\<'into\_return\>

where [Oklcha](../prelude/struct.Oklcha.html): 'into\_return,

 ####

</summary>

Converts [`Self`](../reflect/func/trait.IntoReturn.html) into a [`Return`](../reflect/func/enum.Return.html) value.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#166)[§](#impl-Luminance-for-Oklcha)

### impl [Luminance](../prelude/trait.Luminance.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#168)[§](#method.with_luminance)

#### fn [with\_luminance](../prelude/trait.Luminance.html#tymethod.with_luminance)(&self, lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Return a new version of this color with the given luminance. The resulting color will
be clamped to the valid range for the color space; for some color spaces, clamping
may cause the hue or chroma to change.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#172)[§](#method.luminance)

#### fn [luminance](../prelude/trait.Luminance.html#tymethod.luminance)(&self) -\> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html) ####

</summary>

Return the luminance of this color (0.0 - 1.0).

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#176)[§](#method.darker)

#### fn [darker](../prelude/trait.Luminance.html#tymethod.darker)(&self, amount: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Return a darker version of this color. The `amount` should be between 0.0 and 1.0.
The amount represents an absolute decrease in luminance, and is distributive:`color.darker(a).darker(b) == color.darker(a + b)`. Colors are clamped to black
if the amount would cause them to go below black. [Read more](../prelude/trait.Luminance.html#tymethod.darker)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#185)[§](#method.lighter)

#### fn [lighter](../prelude/trait.Luminance.html#tymethod.lighter)(&self, amount: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Return a lighter version of this color. The `amount` should be between 0.0 and 1.0.
The amount represents an absolute increase in luminance, and is distributive:`color.lighter(a).lighter(b) == color.lighter(a + b)`. Colors are clamped to white
if the amount would cause them to go above white. [Read more](../prelude/trait.Luminance.html#tymethod.lighter)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#114)[§](#impl-Mix-for-Oklcha)

### impl [Mix](../prelude/trait.Mix.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#116)[§](#method.mix)

#### fn [mix](../prelude/trait.Mix.html#tymethod.mix)(&self, other: &[Oklcha](../prelude/struct.Oklcha.html), factor: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -\> [Oklcha](../prelude/struct.Oklcha.html) ####

</summary>

Linearly interpolate between this and another color, by factor.
Factor should be between 0.0 and 1.0.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#40)[§](#method.mix_assign)

#### fn [mix\_assign](../prelude/trait.Mix.html#method.mix_assign)(&mut self, other: Self, factor: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) ####

</summary>

Linearly interpolate between this and another color, by factor, storing the result
in this color. Factor should be between 0.0 and 1.0.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#14)[§](#impl-PartialEq-for-Oklcha)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#14)[§](#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Oklcha](../prelude/struct.Oklcha.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) ####

</summary>

Tests for `self` and `other` values to be equal, and is used by `==`.

</details><details class="toggle method-toggle" open=""><summary>

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262)[§](#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) ####

</summary>

Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-PartialReflect-for-Oklcha)

### impl [PartialReflect](../prelude/trait.PartialReflect.html) for [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.get_represented_type_info)

#### fn [get\_represented\_type\_info](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&'static [TypeInfo](../reflect/enum.TypeInfo.html)\> ####

</summary>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html) of the type *represented* by this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.try_apply)

#### fn [try\_apply](../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html) + 'static),
) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html)\> ####

</summary>

Tries to [`apply`](../prelude/trait.PartialReflect.html#method.apply) a reflected value to this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.try_apply)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.reflect_kind)

#### fn [reflect\_kind](../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -\> [ReflectKind](../reflect/enum.ReflectKind.html) ####

</summary>

Returns a zero-sized enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#method.reflect_kind)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.reflect_ref)

#### fn [reflect\_ref](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -\> [ReflectRef](../reflect/enum.ReflectRef.html)\<'\_\> ####

</summary>

Returns an immutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.reflect_mut)

#### fn [reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -\> [ReflectMut](../reflect/enum.ReflectMut.html)\<'\_\> ####

</summary>

Returns a mutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.reflect_owned)

#### fn [reflect\_owned](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../prelude/struct.Box.html)\<[Oklcha](../prelude/struct.Oklcha.html)\>) -\> [ReflectOwned](../reflect/enum.ReflectOwned.html) ####

</summary>

Returns an owned enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.try_into_reflect)

#### fn [try\_into\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../prelude/struct.Box.html)\<[Oklcha](../prelude/struct.Oklcha.html)\>,
) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<[Box](../prelude/struct.Box.html)\<dyn [Reflect](../prelude/trait.Reflect.html)\>, [Box](../prelude/struct.Box.html)\<dyn [PartialReflect](../prelude/trait.PartialReflect.html)\>\> ####

</summary>

Attempts to cast this type to a boxed, [fully-reflected](../prelude/trait.Reflect.html) value.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.try_as_reflect)

#### fn [try\_as\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&(dyn [Reflect](../prelude/trait.Reflect.html) + 'static)\> ####

</summary>

Attempts to cast this type to a [fully-reflected](../prelude/trait.Reflect.html) value.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.try_as_reflect_mut)

#### fn [try\_as\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&mut (dyn [Reflect](../prelude/trait.Reflect.html) + 'static)\> ####

</summary>

Attempts to cast this type to a mutable, [fully-reflected](../prelude/trait.Reflect.html) value.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.into_partial_reflect)

#### fn [into\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../prelude/struct.Box.html)\<[Oklcha](../prelude/struct.Oklcha.html)\>) -\> [Box](../prelude/struct.Box.html)\<dyn [PartialReflect](../prelude/trait.PartialReflect.html)\> ####

</summary>

Casts this type to a boxed, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.as_partial_reflect)

#### fn [as\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -\> &(dyn [PartialReflect](../prelude/trait.PartialReflect.html) + 'static) ####

</summary>

Casts this type to a reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.as_partial_reflect_mut)

#### fn [as\_partial\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -\> &mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html) + 'static) ####

</summary>

Casts this type to a mutable, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#18)[§](#method.reflect_partial_eq)

#### fn [reflect\_partial\_eq](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html) + 'static),
) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\> ####

</summary>

Returns a “partial equality” comparison result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#18)[§](#method.reflect_clone)

#### fn [reflect\_clone](../prelude/trait.PartialReflect.html#method.reflect_clone)(&self) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<[Box](../prelude/struct.Box.html)\<dyn [Reflect](../prelude/trait.Reflect.html)\>, [ReflectCloneError](../reflect/enum.ReflectCloneError.html)\> ####

</summary>

Attempts to clone `Self` using reflection. [Read more](../prelude/trait.PartialReflect.html#method.reflect_clone)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#184)[§](#method.apply)

#### fn [apply](../prelude/trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html) + 'static)) ####

</summary>

Applies a reflected value to this value. [Read more](../prelude/trait.PartialReflect.html#method.apply)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#253)[§](#method.clone_value)

#### fn [clone\_value](../prelude/trait.PartialReflect.html#method.clone_value)(&self) -\> [Box](../prelude/struct.Box.html)\<dyn [PartialReflect](../prelude/trait.PartialReflect.html)\> ####

</summary>

👎Deprecated since 0.16.0: to clone reflected values, prefer using `reflect_clone`. To convert reflected values to dynamic ones, use `to_dynamic`.

Clones `Self` into its dynamic representation. [Read more](../prelude/trait.PartialReflect.html#method.clone_value)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#291)[§](#method.to_dynamic)

#### fn [to\_dynamic](../prelude/trait.PartialReflect.html#method.to_dynamic)(&self) -\> [Box](../prelude/struct.Box.html)\<dyn [PartialReflect](../prelude/trait.PartialReflect.html)\> ####

</summary>

Converts this reflected value into its dynamic representation based on its [kind](../prelude/trait.PartialReflect.html#method.reflect_kind). [Read more](../prelude/trait.PartialReflect.html#method.to_dynamic)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#335)[§](#method.reflect_hash)

#### fn [reflect\_hash](../prelude/trait.PartialReflect.html#method.reflect_hash)(&self) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> ####

</summary>

Returns a hash of the value (which includes the type). [Read more](../prelude/trait.PartialReflect.html#method.reflect_hash)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#355)[§](#method.debug)

#### fn [debug](../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html)\<'\_\>) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html)\> ####

</summary>

Debug formatter for the value. [Read more](../prelude/trait.PartialReflect.html#method.debug)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#383)[§](#method.is_dynamic)

#### fn [is\_dynamic](../prelude/trait.PartialReflect.html#method.is_dynamic)(&self) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) ####

</summary>

Indicates whether or not this type is a *dynamic* type. [Read more](../prelude/trait.PartialReflect.html#method.is_dynamic)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-Reflect-for-Oklcha)

### impl [Reflect](../prelude/trait.Reflect.html) for [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.into_any)

#### fn [into\_any](../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html)\<[Oklcha](../prelude/struct.Oklcha.html)\>) -\> [Box](../prelude/struct.Box.html)\<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html)\> ####

</summary>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html). [Read more](../prelude/trait.Reflect.html#tymethod.into_any)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.as_any)

#### fn [as\_any](../prelude/trait.Reflect.html#tymethod.as_any)(&self) -\> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + 'static) ####

</summary>

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html). [Read more](../prelude/trait.Reflect.html#tymethod.as_any)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.as_any_mut)

#### fn [as\_any\_mut](../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -\> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + 'static) ####

</summary>

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html). [Read more](../prelude/trait.Reflect.html#tymethod.as_any_mut)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.into_reflect)

#### fn [into\_reflect](../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html)\<[Oklcha](../prelude/struct.Oklcha.html)\>) -\> [Box](../prelude/struct.Box.html)\<dyn [Reflect](../prelude/trait.Reflect.html)\> ####

</summary>

Casts this type to a boxed, fully-reflected value.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.as_reflect)

#### fn [as\_reflect](../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -\> &(dyn [Reflect](../prelude/trait.Reflect.html) + 'static) ####

</summary>

Casts this type to a fully-reflected value.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.as_reflect_mut)

#### fn [as\_reflect\_mut](../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -\> &mut (dyn [Reflect](../prelude/trait.Reflect.html) + 'static) ####

</summary>

Casts this type to a mutable, fully-reflected value.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.set)

#### fn [set](../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html)\<dyn [Reflect](../prelude/trait.Reflect.html)\>) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html)\<dyn [Reflect](../prelude/trait.Reflect.html)\>\> ####

</summary>

Performs a type-checked assignment of a reflected value to this value. [Read more](../prelude/trait.Reflect.html#tymethod.set)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#20)[§](#impl-Serialize-for-Oklcha)

### impl [Serialize](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#20)[§](#method.serialize)

#### fn [serialize](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)\<\_\_S\>( &self, \_\_serializer: \_\_S,
) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<\<\_\_S as [Serializer](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html)\>::[Ok](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok), \<\_\_S as [Serializer](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html)\>::[Error](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error)\>

where \_\_S: [Serializer](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html),

 ####

</summary>

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-Struct-for-Oklcha)

### impl [Struct](../prelude/trait.Struct.html) for [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.field)

#### fn [field](../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html) + 'static)\> ####

</summary>

Returns a reference to the value of the field named `name` as a `&dyn PartialReflect`.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.field_mut)

#### fn [field\_mut](../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html),
) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html) + 'static)\> ####

</summary>

Returns a mutable reference to the value of the field named `name` as a`&mut dyn PartialReflect`.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.field_at)

#### fn [field\_at](../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html) + 'static)\> ####

</summary>

Returns a reference to the value of the field with index `index` as a`&dyn PartialReflect`.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.field_at_mut)

#### fn [field\_at\_mut](../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html),
) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html) + 'static)\> ####

</summary>

Returns a mutable reference to the value of the field with index `index`as a `&mut dyn PartialReflect`.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.name_at)

#### fn [name\_at](../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> ####

</summary>

Returns the name of the field with index `index`.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.field_len)

#### fn [field\_len](../prelude/trait.Struct.html#tymethod.field_len)(&self) -\> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html) ####

</summary>

Returns the number of fields in the struct.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.iter_fields)

#### fn [iter\_fields](../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -\> [FieldIter](../reflect/struct.FieldIter.html)\<'\_\> [ⓘ](#) ####

</summary>

Returns an iterator over the values of the reflectable fields for this struct.

</details>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.to_dynamic_struct)

#### fn [to\_dynamic\_struct](../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -\> [DynamicStruct](../reflect/struct.DynamicStruct.html) ####

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/struct_trait.rs.html#76)[§](#method.clone_dynamic)

#### fn [clone\_dynamic](../prelude/trait.Struct.html#method.clone_dynamic)(&self) -\> [DynamicStruct](../reflect/struct.DynamicStruct.html) ####

</summary>

👎Deprecated since 0.16.0: use `to_dynamic_struct` instead

Clones the struct into a [`DynamicStruct`](../reflect/struct.DynamicStruct.html).

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/struct_trait.rs.html#90)[§](#method.get_represented_struct_info)

#### fn [get\_represented\_struct\_info](../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&'static [StructInfo](../reflect/struct.StructInfo.html)\> ####

</summary>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html) is not available.

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-TypePath-for-Oklcha)

### impl [TypePath](../prelude/trait.TypePath.html) for [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.type_path)

#### fn [type\_path](../prelude/trait.TypePath.html#tymethod.type_path)() -\> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html) ####

</summary>

Returns the fully qualified path of the underlying type. [Read more](../prelude/trait.TypePath.html#tymethod.type_path)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.short_type_path)

#### fn [short\_type\_path](../prelude/trait.TypePath.html#tymethod.short_type_path)() -\> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html) ####

</summary>

Returns a short, pretty-print enabled path to the type. [Read more](../prelude/trait.TypePath.html#tymethod.short_type_path)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.type_ident)

#### fn [type\_ident](../prelude/trait.TypePath.html#method.type_ident)() -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> ####

</summary>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) if it is [anonymous](../prelude/trait.TypePath.html#anonymity). [Read more](../prelude/trait.TypePath.html#method.type_ident)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.crate_name)

#### fn [crate\_name](../prelude/trait.TypePath.html#method.crate_name)() -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> ####

</summary>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) if it is [anonymous](../prelude/trait.TypePath.html#anonymity). [Read more](../prelude/trait.TypePath.html#method.crate_name)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.module_path)

#### fn [module\_path](../prelude/trait.TypePath.html#method.module_path)() -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> ####

</summary>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) if it is [anonymous](../prelude/trait.TypePath.html#anonymity). [Read more](../prelude/trait.TypePath.html#method.module_path)

</details>

</details><details class="toggle implementors-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#impl-Typed-for-Oklcha)

### impl [Typed](../reflect/trait.Typed.html) for [Oklcha](../prelude/struct.Oklcha.html)

where [Oklcha](../prelude/struct.Oklcha.html): [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromReflect](../prelude/trait.FromReflect.html) + [TypePath](../prelude/trait.TypePath.html) + MaybeTyped + RegisterForReflection,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)[§](#method.type_info)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -\> &'static [TypeInfo](../reflect/enum.TypeInfo.html) ####

</summary>

Returns the compile-time [info](../reflect/enum.TypeInfo.html) for the underlying type.

</details>

</details>

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#14)[§](#impl-Copy-for-Oklcha)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

[Source](https://docs.rs/bevy_color/0.16.2/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#14)[§](#impl-StructuralPartialEq-for-Oklcha)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

Auto Trait Implementations[§](#synthetic-implementations)
----------

[§](#impl-Freeze-for-Oklcha)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

[§](#impl-RefUnwindSafe-for-Oklcha)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

[§](#impl-Send-for-Oklcha)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

[§](#impl-Sync-for-Oklcha)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

[§](#impl-Unpin-for-Oklcha)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

[§](#impl-UnwindSafe-for-Oklcha)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html) for [Oklcha](../prelude/struct.Oklcha.html) ###

Blanket Implementations[§](#blanket-implementations)
----------

<details class="toggle implementors-toggle"><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)[§](#impl-Any-for-T)

### impl\<T\> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)[§](#method.type_id)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -\> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html) ####

</summary>

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/bevy_render/0.16.1/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#680-682)[§](#impl-AsBindGroupShaderType%3CU%3E-for-T)

### impl\<T, U\> [AsBindGroupShaderType](../render/render_resource/trait.AsBindGroupShaderType.html)\<U\> for T

where U: [ShaderType](../render/render_resource/trait.ShaderType.html), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for\<'a\> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html)\<U\>,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_render/0.16.1/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#685)[§](#method.as_bind_group_shader_type)

#### fn [as\_bind\_group\_shader\_type](../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../render/render_asset/struct.RenderAssets.html)\<[GpuImage](../render/texture/struct.GpuImage.html)\>) -\> U ####

</summary>

Return the `T` [`ShaderType`](../render/render_resource/trait.ShaderType.html) for `self`. When used in [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html)derives, it is safe to assume that all images in `self` exist.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209)[§](#impl-Borrow%3CT%3E-for-T)

### impl\<T\> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html)\<T\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211)[§](#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -\> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) ####

</summary>

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217)[§](#impl-BorrowMut%3CT%3E-for-T)

### impl\<T\> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html)\<T\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218)[§](#method.borrow_mut)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -\> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) ####

</summary>

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#441)[§](#impl-CloneToUninit-for-T)

### impl\<T\> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html) for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#443)[§](#method.clone_to_uninit)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut ](https://doc.rust-lang.org/nightly/std/primitive.pointer.html)[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) ####

</summary>

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)[§](#impl-Conv-for-T)

### impl\<T\> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html) for T ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)[§](#method.conv)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)\<T\>(self) -\> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html)\<T\>,

 ####

</summary>

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/khronos-egl/6.0.0/x86_64-unknown-linux-gnu/src/khronos_egl/lib.rs.html#148)[§](#impl-Downcast%3CT%3E-for-T)

### impl\<T\> [Downcast](https://docs.rs/khronos-egl/6.0.0/x86_64-unknown-linux-gnu/khronos_egl/trait.Downcast.html)\<T\> for T ###

</summary>

[Source](https://docs.rs/khronos-egl/6.0.0/x86_64-unknown-linux-gnu/src/khronos_egl/lib.rs.html#149)[§](#method.downcast)

#### fn [downcast](https://docs.rs/khronos-egl/6.0.0/x86_64-unknown-linux-gnu/khronos_egl/trait.Downcast.html#tymethod.downcast)(&self) -\> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) ####

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)[§](#impl-Downcast-for-T)

### impl\<T\> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html) for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)[§](#method.into_any-1)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html)\<T\>) -\> [Box](../prelude/struct.Box.html)\<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html)\> ####

</summary>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be`downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)[§](#method.into_any_rc)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html)\<T\>) -\> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html)\<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html)\> ####

</summary>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further`downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)[§](#method.as_any-1)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -\> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + 'static) ####

</summary>

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot
generate `&Any`’s vtable from `&Trait`’s.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)[§](#method.as_any_mut-1)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -\> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + 'static) ####

</summary>

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot
generate `&mut Any`’s vtable from `&mut Trait`’s.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)[§](#impl-Downcast-for-T-1)

### impl\<T\> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html) for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)[§](#method.into_any-2)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html)\<T\>) -\> [Box](../prelude/struct.Box.html)\<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html)\> ####

</summary>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can
then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)[§](#method.into_any_rc-1)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html)\<T\>) -\> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html)\<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html)\> ####

</summary>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be
further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)[§](#method.as_any-2)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -\> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + 'static) ####

</summary>

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot
generate `&Any`’s vtable from `&Trait`’s.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)[§](#method.as_any_mut-2)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -\> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + 'static) ####

</summary>

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot
generate `&mut Any`’s vtable from `&mut Trait`’s.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)[§](#impl-DowncastSend-for-T)

### impl\<T\> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html) for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)[§](#method.into_any_send)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../prelude/struct.Box.html)\<T\>) -\> [Box](../prelude/struct.Box.html)\<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html)\> ####

</summary>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which
can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)[§](#impl-DowncastSync-for-T)

### impl\<T\> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html) for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)[§](#method.into_any_arc)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../platform/sync/struct.Arc.html)\<T\>) -\> [Arc](../platform/sync/struct.Arc.html)\<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html)\> ####

</summary>

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be
further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)[§](#impl-DynamicTypePath-for-T)

### impl\<T\> [DynamicTypePath](../reflect/trait.DynamicTypePath.html) for T

where T: [TypePath](../prelude/trait.TypePath.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)[§](#method.reflect_type_path)

#### fn [reflect\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -\> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html) ####

</summary>

See [`TypePath::type_path`](../prelude/trait.TypePath.html#tymethod.type_path).

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)[§](#method.reflect_short_type_path)

#### fn [reflect\_short\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -\> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html) ####

</summary>

See [`TypePath::short_type_path`](../prelude/trait.TypePath.html#tymethod.short_type_path).

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)[§](#method.reflect_type_ident)

#### fn [reflect\_type\_ident](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> ####

</summary>

See [`TypePath::type_ident`](../prelude/trait.TypePath.html#method.type_ident).

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)[§](#method.reflect_crate_name)

#### fn [reflect\_crate\_name](../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> ####

</summary>

See [`TypePath::crate_name`](../prelude/trait.TypePath.html#method.crate_name).

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)[§](#method.reflect_module_path)

#### fn [reflect\_module\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> ####

</summary>

See [`TypePath::module_path`](../prelude/trait.TypePath.html#method.module_path).

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#157)[§](#impl-DynamicTyped-for-T)

### impl\<T\> [DynamicTyped](../reflect/trait.DynamicTyped.html) for T

where T: [Typed](../reflect/trait.Typed.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#159)[§](#method.reflect_type_info)

#### fn [reflect\_type\_info](../reflect/trait.DynamicTyped.html#tymethod.reflect_type_info)(&self) -\> &'static [TypeInfo](../reflect/enum.TypeInfo.html) ####

</summary>

See [`Typed::type_info`](../reflect/trait.Typed.html#tymethod.type_info).

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)[§](#impl-FmtForward-for-T)

### impl\<T\> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html) for T ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)[§](#method.fmt_binary)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -\> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html)\<Self\>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html),

 ####

</summary>

Causes `self` to use its `Binary` implementation when `Debug`-formatted.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)[§](#method.fmt_display)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -\> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html)\<Self\>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html),

 ####

</summary>

Causes `self` to use its `Display` implementation when`Debug`-formatted.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)[§](#method.fmt_lower_exp)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -\> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html)\<Self\>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html),

 ####

</summary>

Causes `self` to use its `LowerExp` implementation when`Debug`-formatted.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)[§](#method.fmt_lower_hex)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -\> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html)\<Self\>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html),

 ####

</summary>

Causes `self` to use its `LowerHex` implementation when`Debug`-formatted.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)[§](#method.fmt_octal)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -\> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html)\<Self\>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html),

 ####

</summary>

Causes `self` to use its `Octal` implementation when `Debug`-formatted.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)[§](#method.fmt_pointer)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -\> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html)\<Self\>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html),

 ####

</summary>

Causes `self` to use its `Pointer` implementation when`Debug`-formatted.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)[§](#method.fmt_upper_exp)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -\> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html)\<Self\>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html),

 ####

</summary>

Causes `self` to use its `UpperExp` implementation when`Debug`-formatted.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)[§](#method.fmt_upper_hex)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -\> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html)\<Self\>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html),

 ####

</summary>

Causes `self` to use its `UpperHex` implementation when`Debug`-formatted.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)[§](#method.fmt_list)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -\> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html)\<Self\>

where &'a Self: for\<'a\> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html),

 ####

</summary>

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#773)[§](#impl-From%3CT%3E-for-T)

### impl\<T\> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<T\> for T ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#776)[§](#method.from-20)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -\> T ####

</summary>

Returns the argument unchanged.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)[§](#impl-FromSample%3CS%3E-for-S)

### impl\<S\> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html)\<S\> for S ###

</summary>

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)[§](#method.from_sample_)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -\> S ####

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/bevy_ecs/0.16.1/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3756)[§](#impl-FromWorld-for-T)

### impl\<T\> [FromWorld](../prelude/trait.FromWorld.html) for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_ecs/0.16.1/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3758)[§](#method.from_world)

#### fn [from\_world](../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../prelude/struct.World.html)) -\> T ####

</summary>

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default).

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/struct_trait.rs.html#256)[§](#impl-GetField-for-S)

### impl\<S\> [GetField](../prelude/trait.GetField.html) for S

where S: [Struct](../prelude/trait.Struct.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/struct_trait.rs.html#257)[§](#method.get_field)

#### fn [get\_field](../prelude/trait.GetField.html#tymethod.get_field)\<T\>(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../prelude/trait.Reflect.html),

 ####

</summary>

Returns a reference to the value of the field named `name`, downcast to`T`.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/struct_trait.rs.html#262)[§](#method.get_field_mut)

#### fn [get\_field\_mut](../prelude/trait.GetField.html#tymethod.get_field_mut)\<T\>(&mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../prelude/trait.Reflect.html),

 ####

</summary>

Returns a mutable reference to the value of the field named `name`,
downcast to `T`.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#294)[§](#impl-GetPath-for-T)

### impl\<T\> [GetPath](../prelude/trait.GetPath.html) for T

where T: [Reflect](../prelude/trait.Reflect.html) + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#255)[§](#method.reflect_path)

#### fn [reflect\_path](../prelude/trait.GetPath.html#method.reflect_path)\<'p\>( &self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html)\<'p\>,
) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html) + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html)\<'p\>\> ####

</summary>

Returns a reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.reflect_path)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#263-266)[§](#method.reflect_path_mut)

#### fn [reflect\_path\_mut](../prelude/trait.GetPath.html#method.reflect_path_mut)\<'p\>( &mut self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html)\<'p\>,
) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html) + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html)\<'p\>\> ####

</summary>

Returns a mutable reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.reflect_path_mut)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#277)[§](#method.path)

#### fn [path](../prelude/trait.GetPath.html#method.path)\<'p, T\>( &self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html)\<'p\>,
) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html)\<'p\>\>

where T: [Reflect](../prelude/trait.Reflect.html),

 ####

</summary>

Returns a statically typed reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.path)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#288)[§](#method.path_mut)

#### fn [path\_mut](../prelude/trait.GetPath.html#method.path_mut)\<'p, T\>( &mut self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html)\<'p\>,
) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html)\<'p\>\>

where T: [Reflect](../prelude/trait.Reflect.html),

 ####

</summary>

Returns a statically typed mutable reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.path_mut)

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/tracing/0.1.41/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)[§](#impl-Instrument-for-T)

### impl\<T\> [Instrument](../log/tracing/trait.Instrument.html) for T ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tracing/0.1.41/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)[§](#method.instrument)

#### fn [instrument](../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../log/tracing/struct.Span.html)) -\> [Instrumented](../log/tracing/instrument/struct.Instrumented.html)\<Self\> [ⓘ](#) ####

</summary>

Instruments this type with the provided [`Span`](../log/tracing/struct.Span.html), returning an`Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.instrument)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tracing/0.1.41/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)[§](#method.in_current_span)

#### fn [in\_current\_span](../log/tracing/trait.Instrument.html#method.in_current_span)(self) -\> [Instrumented](../log/tracing/instrument/struct.Instrumented.html)\<Self\> [ⓘ](#) ####

</summary>

Instruments this type with the [current](../log/tracing/struct.Span.html#method.current) [`Span`](../log/tracing/struct.Span.html), returning an`Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.in_current_span)

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#756-758)[§](#impl-Into%3CU%3E-for-T)

### impl\<T, U\> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html)\<U\> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)\<T\>,

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#766)[§](#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -\> U ####

</summary>

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of`[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)<T> for U` chooses to do.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)[§](#impl-IntoEither-for-T)

### impl\<T\> [IntoEither](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html) for T ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)[§](#method.into_either)

#### fn [into\_either](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -\> [Either](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)\<Self, Self\> [ⓘ](#) ####

</summary>

Converts `self` into a [`Left`](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if `into_left` is `true`.
Converts `self` into a [`Right`](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise. [Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)[§](#method.into_either_with)

#### fn [into\_either\_with](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)\<F\>(self, into\_left: F) -\> [Either](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)\<Self, Self\> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)(&Self) -\> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

 ####

</summary>

Converts `self` into a [`Left`](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if `into_left(&self)` returns `true`.
Converts `self` into a [`Right`](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise. [Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/symphonia-core/0.5.4/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)[§](#impl-IntoSample%3CT%3E-for-F)

### impl\<F, T\> [IntoSample](https://docs.rs/symphonia-core/0.5.4/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html)\<T\> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.4/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html)\<F\>,

 ###

</summary>

[Source](https://docs.rs/symphonia-core/0.5.4/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)[§](#method.into_sample)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.4/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -\> T ####

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/zvariant/5.5.3/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)[§](#impl-NoneValue-for-T)

### impl\<T\> [NoneValue](https://docs.rs/zvariant/5.5.3/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html) for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html),

 ###

</summary>

[Source](https://docs.rs/zvariant/5.5.3/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)[§](#associatedtype.NoneType)

#### type [NoneType](https://docs.rs/zvariant/5.5.3/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T ####

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/zvariant/5.5.3/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)[§](#method.null_value)

#### fn [null\_value](https://docs.rs/zvariant/5.5.3/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -\> T ####

</summary>

The none-equivalent value.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)[§](#impl-Pipe-for-T)

### impl\<T\> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html) for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)[§](#method.pipe)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)\<R\>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)(Self) -\> R) -\> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ####

</summary>

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)[§](#method.pipe_ref)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)\<'a, R\>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)(&'a Self) -\> R) -\> R

where R: 'a,

 ####

</summary>

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)[§](#method.pipe_ref_mut)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)\<'a, R\>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)(&'a mut Self) -\> R) -\> R

where R: 'a,

 ####

</summary>

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)[§](#method.pipe_borrow)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)\<'a, B, R\>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> R) -\> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html)\<B\>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html), R: 'a,

 ####

</summary>

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)[§](#method.pipe_borrow_mut)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)\<'a, B, R\>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> R,
) -\> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html)\<B\>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html), R: 'a,

 ####

</summary>

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe
function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)[§](#method.pipe_as_ref)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)\<'a, U, R\>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> R) -\> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html)\<U\>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html), R: 'a,

 ####

</summary>

Borrows `self`, then passes `self.as_ref()` into the pipe function.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)[§](#method.pipe_as_mut)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)\<'a, U, R\>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> R) -\> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html)\<U\>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html), R: 'a,

 ####

</summary>

Mutably borrows `self`, then passes `self.as_mut()` into the pipe
function.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)[§](#method.pipe_deref)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)\<'a, T, R\>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> R) -\> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html)\<Target = T\>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html), R: 'a,

 ####

</summary>

Borrows `self`, then passes `self.deref()` into the pipe function.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)[§](#method.pipe_deref_mut)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)\<'a, T, R\>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> R,
) -\> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html)\<Target = T\> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html), R: 'a,

 ####

</summary>

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe
function.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#194)[§](#impl-Pointable-for-T)

### impl\<T\> [Pointable](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html) for T ###

</summary>

<details class="toggle" open=""><summary>

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#195)[§](#associatedconstant.ALIGN)

#### const [ALIGN](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#associatedconstant.ALIGN): [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html) ####

</summary>

The alignment of pointer.

</details><details class="toggle" open=""><summary>

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#197)[§](#associatedtype.Init)

#### type [Init](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#associatedtype.Init) = T ####

</summary>

The type for initializers.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#199)[§](#method.init)

#### unsafe fn [init](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.init)(init: \<T as [Pointable](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html)\>::[Init](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#associatedtype.Init)) -\> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html) ####

</summary>

Initializes a with the given initializer. [Read more](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.init)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#203)[§](#method.deref)

#### unsafe fn [deref](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.deref)\<'a\>(ptr: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -\> [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) ####

</summary>

Dereferences the given pointer. [Read more](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.deref)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#207)[§](#method.deref_mut)

#### unsafe fn [deref\_mut](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.deref_mut)\<'a\>(ptr: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -\> [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) ####

</summary>

Mutably dereferences the given pointer. [Read more](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.deref_mut)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/src/crossbeam_epoch/atomic.rs.html#211)[§](#method.drop)

#### unsafe fn [drop](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.drop)(ptr: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) ####

</summary>

Drops the object pointed to by the given pointer. [Read more](https://docs.rs/crossbeam-epoch/0.9.18/x86_64-unknown-linux-gnu/crossbeam_epoch/atomic/trait.Pointable.html#tymethod.drop)

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/lebe/0.5.2/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#347)[§](#impl-ReadPrimitive%3CR%3E-for-P)

### impl\<R, P\> [ReadPrimitive](https://docs.rs/lebe/0.5.2/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html)\<R\> for P

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html) + [ReadEndian](https://docs.rs/lebe/0.5.2/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html)\<P\>, P: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html),

 ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/lebe/0.5.2/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#377)[§](#method.read_from_little_endian)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.2/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<Self, [Error](../tasks/futures_lite/io/struct.Error.html)\> ####

</summary>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/lebe/0.5.2/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)[§](#method.read_from_big_endian)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.2/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<Self, [Error](../tasks/futures_lite/io/struct.Error.html)\> ####

</summary>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/lebe/0.5.2/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)[§](#method.read_from_native_endian)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.2/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<Self, [Error](../tasks/futures_lite/io/struct.Error.html)\> ####

</summary>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/erased-serde/0.4.6/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#224-226)[§](#impl-Serialize-for-T)

### impl\<T\> [Serialize](../reflect/erased_serde/trait.Serialize.html) for T

where T: [Serialize](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html) + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ###

</summary>

[Source](https://docs.rs/erased-serde/0.4.6/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#228)[§](#method.erased_serialize)

#### fn [erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html)) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../reflect/erased_serde/struct.Error.html)\> ####

[Source](https://docs.rs/erased-serde/0.4.6/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#236)[§](#method.do_erased_serialize)

#### fn [do\_erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html),
) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl\> ####

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)[§](#impl-Tap-for-T)

### impl\<T\> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html) for T ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)[§](#method.tap)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)(&Self)) -\> Self ####

</summary>

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)[§](#method.tap_mut)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)(&mut Self)) -\> Self ####

</summary>

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)[§](#method.tap_borrow)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)\<B\>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -\> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html)\<B\>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ####

</summary>

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)[§](#method.tap_borrow_mut)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)\<B\>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -\> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html)\<B\>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ####

</summary>

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)[§](#method.tap_ref)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)\<R\>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -\> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html)\<R\>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ####

</summary>

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)[§](#method.tap_ref_mut)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)\<R\>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -\> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html)\<R\>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ####

</summary>

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)[§](#method.tap_deref)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)\<T\>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -\> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html)\<Target = T\>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ####

</summary>

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)[§](#method.tap_deref_mut)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)\<T\>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -\> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html)\<Target = T\> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ####

</summary>

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)[§](#method.tap_dbg)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)(&Self)) -\> Self ####

</summary>

Calls `.tap()` only in debug builds, and is erased in release builds.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)[§](#method.tap_mut_dbg)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)(&mut Self)) -\> Self ####

</summary>

Calls `.tap_mut()` only in debug builds, and is erased in release
builds.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)[§](#method.tap_borrow_dbg)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)\<B\>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -\> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html)\<B\>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ####

</summary>

Calls `.tap_borrow()` only in debug builds, and is erased in release
builds.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)[§](#method.tap_borrow_mut_dbg)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)\<B\>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -\> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html)\<B\>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ####

</summary>

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release
builds.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)[§](#method.tap_ref_dbg)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)\<R\>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -\> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html)\<R\>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ####

</summary>

Calls `.tap_ref()` only in debug builds, and is erased in release
builds.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)[§](#method.tap_ref_mut_dbg)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)\<R\>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -\> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html)\<R\>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ####

</summary>

Calls `.tap_ref_mut()` only in debug builds, and is erased in release
builds.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)[§](#method.tap_deref_dbg)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)\<T\>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -\> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html)\<Target = T\>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ####

</summary>

Calls `.tap_deref()` only in debug builds, and is erased in release
builds.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)[§](#method.tap_deref_mut_dbg)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)\<T\>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html)([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -\> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html)\<Target = T\> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html),

 ####

</summary>

Calls `.tap_deref_mut()` only in debug builds, and is erased in release
builds.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84)[§](#impl-ToOwned-for-T)

### impl\<T\> [ToOwned](../prelude/trait.ToOwned.html) for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html),

 ###

</summary>

<details class="toggle" open=""><summary>

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86)[§](#associatedtype.Owned)

#### type [Owned](../prelude/trait.ToOwned.html#associatedtype.Owned) = T ####

</summary>

The resulting type after obtaining ownership.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87)[§](#method.to_owned)

#### fn [to\_owned](../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -\> T ####

</summary>

Creates owned data from borrowed data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#tymethod.to_owned)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91)[§](#method.clone_into)

#### fn [clone\_into](../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) ####

</summary>

Uses borrowed data to replace owned data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#method.clone_into)

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)[§](#impl-ToSample%3CU%3E-for-T)

### impl\<T, U\> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html)\<U\> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html)\<T\>,

 ###

</summary>

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)[§](#method.to_sample_)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -\> U ####

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)[§](#impl-TryConv-for-T)

### impl\<T\> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html) for T ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)[§](#method.try_conv)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)\<T\>(self) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error)\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html)\<T\>,

 ####

</summary>

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#812-814)[§](#impl-TryFrom%3CU%3E-for-T)

### impl\<T, U\> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html)\<U\> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html)\<T\>,

 ###

</summary>

<details class="toggle" open=""><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#816)[§](#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html) ####

</summary>

The type returned in the event of a conversion error.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#819)[§](#method.try_from)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<T, \<T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html)\<U\>\>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error)\> ####

</summary>

Performs the conversion.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#797-799)[§](#impl-TryInto%3CU%3E-for-T)

### impl\<T, U\> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html)\<U\> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html)\<T\>,

 ###

</summary>

<details class="toggle" open=""><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#801)[§](#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = \<U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html)\<T\>\>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) ####

</summary>

The type returned in the event of a conversion error.

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#804)[§](#method.try_into)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -\> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)\<U, \<U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html)\<T\>\>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error)\> ####

</summary>

Performs the conversion.

</details>

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#691-693)[§](#impl-TypeData-for-T)

### impl\<T\> [TypeData](../reflect/trait.TypeData.html) for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html) + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html),

 ###

</summary>

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#695)[§](#method.clone_type_data)

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -\> [Box](../prelude/struct.Box.html)\<dyn [TypeData](../reflect/trait.TypeData.html)\> ####

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/khronos-egl/6.0.0/x86_64-unknown-linux-gnu/src/khronos_egl/lib.rs.html#158)[§](#impl-Upcast%3CT%3E-for-T)

### impl\<T\> [Upcast](https://docs.rs/khronos-egl/6.0.0/x86_64-unknown-linux-gnu/khronos_egl/trait.Upcast.html)\<T\> for T ###

</summary>

[Source](https://docs.rs/khronos-egl/6.0.0/x86_64-unknown-linux-gnu/src/khronos_egl/lib.rs.html#159)[§](#method.upcast)

#### fn [upcast](https://docs.rs/khronos-egl/6.0.0/x86_64-unknown-linux-gnu/khronos_egl/trait.Upcast.html#tymethod.upcast)(&self) -\> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)\<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\> ####

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)[§](#impl-VZip%3CV%3E-for-T)

### impl\<V, T\> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html)\<V\> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html)\<T\>,

 ###

</summary>

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)[§](#method.vzip)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -\> V ####

</details><details class="toggle implementors-toggle"><summary>

[Source](https://docs.rs/tracing/0.1.41/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)[§](#impl-WithSubscriber-for-T)

### impl\<T\> [WithSubscriber](../log/tracing/instrument/trait.WithSubscriber.html) for T ###

</summary>

<details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tracing/0.1.41/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)[§](#method.with_subscriber)

#### fn [with\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)\<S\>(self, subscriber: S) -\> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html)\<Self\> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html)\<[Dispatch](../log/tracing/struct.Dispatch.html)\>,

 ####

</summary>

Attaches the provided [`Subscriber`](../log/tracing/trait.Subscriber.html) to this type, returning a[`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html) wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

</details><details class="toggle method-toggle" open=""><summary>

[Source](https://docs.rs/tracing/0.1.41/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)[§](#method.with_current_subscriber)

#### fn [with\_current\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -\> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html)\<Self\> [ⓘ](#) ####

</summary>

Attaches the current [default](../log/tracing/dispatcher/index.html#setting-the-default-subscriber) [`Subscriber`](../log/tracing/trait.Subscriber.html) to this type, returning a[`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html) wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

</details>

</details>

[Source](https://docs.rs/bevy_tasks/0.16.1/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#25)[§](#impl-ConditionalSend-for-T)

### impl\<T\> [ConditionalSend](../tasks/trait.ConditionalSend.html) for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html),

 ###

[Source](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/src/serde/de/mod.rs.html#614)[§](#impl-DeserializeOwned-for-T)

### impl\<T\> [DeserializeOwned](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.DeserializeOwned.html) for T

where T: for\<'de\> [Deserialize](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html)\<'de\>,

 ###

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)[§](#impl-Duplex%3CS%3E-for-T)

### impl\<S, T\> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html)\<S\> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html)\<S\> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html)\<S\>,

 ###

[Source](https://docs.rs/bevy_reflect/0.16.1/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)[§](#impl-Reflectable-for-T)

### impl\<T\> [Reflectable](../reflect/trait.Reflectable.html) for T

where T: [Reflect](../prelude/trait.Reflect.html) + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html) + [Typed](../reflect/trait.Typed.html) + [TypePath](../prelude/trait.TypePath.html),

 ###

[Source](https://docs.rs/bevy_asset/0.16.1/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#173)[§](#impl-Settings-for-T)

### impl\<T\> [Settings](../asset/meta/trait.Settings.html) for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html),

 ###

[Source](https://docs.rs/wgpu-types/24.0.0/x86_64-unknown-linux-gnu/src/wgpu_types/lib.rs.html#7915)[§](#impl-WasmNotSend-for-T)

### impl\<T\> [WasmNotSend](https://docs.rs/wgpu-types/24.0.0/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html) for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html),

 ###

[Source](https://docs.rs/wgpu-types/24.0.0/x86_64-unknown-linux-gnu/src/wgpu_types/lib.rs.html#7899)[§](#impl-WasmNotSendSync-for-T)

### impl\<T\> [WasmNotSendSync](https://docs.rs/wgpu-types/24.0.0/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html) for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/24.0.0/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html) + [WasmNotSync](https://docs.rs/wgpu-types/24.0.0/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html),

 ###

[Source](https://docs.rs/wgpu-types/24.0.0/x86_64-unknown-linux-gnu/src/wgpu_types/lib.rs.html#7948)[§](#impl-WasmNotSync-for-T)

### impl\<T\> [WasmNotSync](https://docs.rs/wgpu-types/24.0.0/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html) for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html),

 ###

{"Either\<Self, Self\>":"\<h3\>Notable traits for \<code\>\<a class=\\"enum\\" href=\\"https://docs.rs/either/1.15.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\"\>Either\</a\>&lt;L, R&gt;\</code\>\</h3\>\<pre\>\<code\>\<div class=\\"where\\"\>impl&lt;L, R&gt; \<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\"\>Iterator\</a\> for \<a class=\\"enum\\" href=\\"https://docs.rs/either/1.15.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\"\>Either\</a\>&lt;L, R&gt;\<div class=\\"where\\"\>where\\n L: \<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\"\>Iterator\</a\>,\\n R: \<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\"\>Iterator\</a\>&lt;Item = &lt;L as \<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\"\>Iterator\</a\>&gt;::\<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\"\>Item\</a\>&gt;,\</div\>\</div\>\<div class=\\"where\\"\> type \<a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\"\>Item\</a\> = &lt;L as \<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\"\>Iterator\</a\>&gt;::\<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\"\>Item\</a\>;\</div\>\<div class=\\"where\\"\>impl&lt;L, R&gt; \<a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\"\>Future\</a\> for \<a class=\\"enum\\" href=\\"https://docs.rs/either/1.15.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\"\>Either\</a\>&lt;L, R&gt;\<div class=\\"where\\"\>where\\n L: \<a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\"\>Future\</a\>,\\n R: \<a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\"\>Future\</a\>&lt;Output = &lt;L as \<a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\"\>Future\</a\>&gt;::\<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\"\>Output\</a\>&gt;,\</div\>\</div\>\<div class=\\"where\\"\> type \<a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\"\>Output\</a\> = &lt;L as \<a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\"\>Future\</a\>&gt;::\<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\"\>Output\</a\>;\</div\>","FieldIter\<'\_\>":"\<h3\>Notable traits for \<code\>\<a class=\\"struct\\" href=\\"../reflect/struct.FieldIter.html\\" title=\\"struct bevy::reflect::FieldIter\\"\>FieldIter\</a\>&lt;'a&gt;\</code\>\</h3\>\<pre\>\<code\>\<div class=\\"where\\"\>impl&lt;'a&gt; \<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\"\>Iterator\</a\> for \<a class=\\"struct\\" href=\\"../reflect/struct.FieldIter.html\\" title=\\"struct bevy::reflect::FieldIter\\"\>FieldIter\</a\>&lt;'a&gt;\</div\>\<div class=\\"where\\"\> type \<a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\"\>Item\</a\> = &amp;'a (dyn \<a class=\\"trait\\" href=\\"../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\"\>PartialReflect\</a\> + 'static);\</div\>","Instrumented\<Self\>":"\<h3\>Notable traits for \<code\>\<a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\"\>Instrumented\</a\>&lt;T&gt;\</code\>\</h3\>\<pre\>\<code\>\<div class=\\"where\\"\>impl&lt;T&gt; \<a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\"\>Future\</a\> for \<a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\"\>Instrumented\</a\>&lt;T&gt;\<div class=\\"where\\"\>where\\n T: \<a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\"\>Future\</a\>,\</div\>\</div\>\<div class=\\"where\\"\> type \<a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\"\>Output\</a\> = &lt;T as \<a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\"\>Future\</a\>&gt;::\<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\"\>Output\</a\>;\</div\>","WithDispatch\<Self\>":"\<h3\>Notable traits for \<code\>\<a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\"\>WithDispatch\</a\>&lt;T&gt;\</code\>\</h3\>\<pre\>\<code\>\<div class=\\"where\\"\>impl&lt;T&gt; \<a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\"\>Future\</a\> for \<a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\"\>WithDispatch\</a\>&lt;T&gt;\<div class=\\"where\\"\>where\\n T: \<a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\"\>Future\</a\>,\</div\>\</div\>\<div class=\\"where\\"\> type \<a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\"\>Output\</a\> = &lt;T as \<a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\"\>Future\</a\>&gt;::\<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\"\>Output\</a\>;\</div\>"}

 // Adds tags to documentation pages for common Bevy traits like `Component` or `Resource`. // This makes it easier to see at a glance what types are used for. // // This extension is passed to `rustdoc` using the `--html-after-content` flag. // Traits that we want to show as tags. // Order determines sort order of items in listings. const bevyTraits = [ 'Plugin', 'PluginGroup', 'Component', 'Resource', 'Asset', 'Event', 'ScheduleLabel', 'SystemSet', 'SystemParam', 'Relationship', 'RelationshipTarget' ]; // Find all traits that are implemented by the current type. const implementedBevyTraits = findImplementedBevyTraits(document); // If we found any implemented traits, add them as tags to the top of the page. if (implementedBevyTraits.size \> 0) { // Create a container for the tags. const heading = document.body.querySelector(".main-heading h1"); const tagContainer = document.createElement('div'); tagContainer.className = 'bevy-tag-container'; heading.appendChild(tagContainer); // Check if an implemented trait has a `type Mutability = Immutable` associated type. // This is used to determine if a `Component` is immutable or not. // TODO: Ideally we should just check the associated types of the `Component` trait, // but the docs.rs layout makes it tricky to do so in a robust way. const associatedTypeHeader = document.querySelectorAll(".trait-impl.associatedtype .code-header"); const isImmutable = [...associatedTypeHeader].some(el =\> el.innerText.includes('type Mutability = Immutable')); // Create a tag for each implemented trait. for (let [tagName, href] of implementedBevyTraits) { if (tagName == 'Component' & isImmutable) { tagName = 'Immutable Component'; } // Create the tag and append it to the container. tagContainer.appendChild(createBevyTag(tagName, href)); } } function findImplementedBevyTraits(doc) { // Traits that are implemented by the current type. // The key is the trait name, and the value is the href to the trait's documentation. const implementedTraits = new Map(); // Find all trait implementation headers. const allTraitHeaders = doc.body.querySelectorAll( '#trait-implementations-list .impl .code-header, #blanket-implementations-list .impl .code-header' ); for (const header of allTraitHeaders) { // We can extract the trait name by removing any generics and splitting the string by spaces. // This results in ['impl', 'TraitName', 'for', 'TypeName']. const traitName = removeGenerics(header.innerText).split(' ')[1].trim(); // Find the link to the trait if the anchor element exists. // Otherwise, the trait is just in plain text. const traitLinkEl = [...header.children].find(el =\> el.getAttribute('href')?.includes(`trait.${traitName}.html`)); const href = traitLinkEl?.getAttribute('href'); implementedTraits.set(traitName, href); } const implementedBevyTraits = new Map( [...implementedTraits].filter(([traitName, \_]) =\> bevyTraits.find((x) =\> x == traitName)) ); return implementedBevyTraits; } // Helper function to remove generics from a string of Rust code. // For example, 'Vec\<T\>' would become 'Vec'. function removeGenerics(str) { // Remove the innermost generics. const newStr = str.replace(/\<([^\<\>])\*\>/g, ''); // If there are still generics, perform the removal again recursively. if (newStr !== str) { return removeGenerics(newStr); } // No more generics to remove. return newStr; } // Helper function to create a tag element with the given name and href, // if available. function createBevyTag(tagName, href) { const el = document.createElement('a'); const kebabCaseName = tagName.toLowerCase().replace(' ', '-'); if (href) { el.setAttribute('href', href); } el.innerText = tagName; el.className = `bevy-tag ${kebabCaseName}-tag`; return el; }  .bevy-tag-container { padding: 0.5rem 0; display: flex; flex-wrap: wrap; gap: 0.5rem; } .bevy-tag { display: flex; align-items: center; width: fit-content; height: 1.5rem; padding: 0 0.5rem; border-radius: 0.75rem; font-size: 1rem; font-weight: normal; color: white; } .bevy-tag { background-color: var(--tag-color); } .component-tag, .immutable-component-tag { --tag-color: oklch(50% 27% 95); } .resource-tag { --tag-color: oklch(50% 27% 110); } .asset-tag { --tag-color: oklch(50% 27% 0); } .event-tag { --tag-color: oklch(50% 27% 310); } .plugin-tag, .plugingroup-tag { --tag-color: oklch(50% 27% 50); } .schedulelabel-tag, .systemset-tag { --tag-color: oklch(50% 27% 270); } .systemparam-tag { --tag-color: oklch(50% 27% 200); } .relationship-tag, .relationshiptarget-tag { --tag-color: oklch(50% 27% 150); }

--- END RESULT ---
"###;
}
