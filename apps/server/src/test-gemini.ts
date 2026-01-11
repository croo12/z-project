import "dotenv/config";
import { createChatModel } from "@z-project/ai";
import { StringOutputParser } from "@langchain/core/output_parsers";
import path from "path";

async function main() {
  console.log("--- Gemini Test Start ---");
  console.log("CWD:", process.cwd());
  // Manually load .env to be sure
  const envPath = path.resolve(process.cwd(), "apps/server/.env");
  console.log("Loading .env from:", envPath);
  console.log("Loading .env from:", envPath);
  // require('dotenv').config({ path: envPath }); // Removed to fix ESM error
  // Already imported 'dotenv/config' at top, but if we need specific path:
  import("dotenv").then((d) => d.config({ path: envPath }));

  console.log("GEMINI_API_KEY present:", !!process.env.GEMINI_API_KEY);
  console.log("GEMINI_API_KEY length:", process.env.GEMINI_API_KEY?.length);

  try {
    const model = createChatModel({
      provider: "gemini",
      modelName: "gemini-1.5-flash",
      temperature: 0,
      apiKey: process.env.GEMINI_API_KEY, // Explicit pass
    });

    console.log("Model initialized. Invoking...");
    const response = await model
      .pipe(new StringOutputParser())
      .invoke("Hello, simple test.");
    console.log("--- Response ---");
    console.log(response);
    console.log("--- End ---");
  } catch (error: any) {
    console.error("--- Error Details ---");
    console.error("Name:", error.name);
    console.error("Message:", error.message);
    if (error.cause) console.error("Cause:", error.cause);
    if (error.response)
      console.error("Response:", JSON.stringify(error.response, null, 2));
    console.error("Full Error:", error);
  }
}

main();
