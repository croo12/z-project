import "dotenv/config";
import path from "path";
import fs from "fs";

async function debugGemini() {
  // 1. Load Environment Variables
  const envPath = path.resolve(process.cwd(), "apps/server/.env");
  if (fs.existsSync(envPath)) {
    require("dotenv").config({ path: envPath });
  }

  const apiKey = process.env.GEMINI_API_KEY;
  console.log("--- Gemini Debug ---");
  console.log(`API Key Length: ${apiKey?.length || 0}`);

  if (!apiKey) {
    console.error("CRITICAL: No API Key found.");
    return;
  }

  // 2. Direct API Call to List Models (No libraries)
  // This verifies what models this specific API key can actually see.
  const baseUrl = "https://generativelanguage.googleapis.com/v1beta/models";
  const listModelsUrl = `${baseUrl}?key=${apiKey}`;

  console.log(`Checking available models via: ${baseUrl}`);

  try {
    const response = await fetch(listModelsUrl);

    if (!response.ok) {
      console.error(`HTTP Error: ${response.status} ${response.statusText}`);
      const text = await response.text();
      console.error("Response Body:", text);
      return;
    }

    const data = await response.json();
    console.log("--- Available Models ---");
    if (data.models) {
      // Filter for 'generateContent' supported models and print short names
      const chatModels = data.models
        .filter((m: any) =>
          m.supportedGenerationMethods.includes("generateContent")
        )
        .map((m: any) => m.name.replace("models/", ""));

      console.log(chatModels.join(", "));

      if (chatModels.length > 0) {
        console.log(`\nRecommanded Model: ${chatModels[0]}`);
      }
    } else {
      console.log("No models found in response.");
      console.log(JSON.stringify(data, null, 2));
    }
  } catch (error) {
    console.error("Network/Fetch Error:", error);
  }
}

debugGemini();
