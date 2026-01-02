# Android Keystore Setup for Signed Releases

## Goal
To publish the app on the Google Play Store or distribute it as a secure release, the APK/AAB must be signed with a secure keystore.

## Prerequisites
- Android Studio or Java JDK installed (`keytool` command needed).
- GitHub Repository Admin access (to set Secrets).

## Steps

### 1. Generate a Keystore
Run the following command in your terminal to generate a keystore file (`release.jks`).
**Keep your password and alias safe!**

```bash
keytool -genkey -v -keystore release.jks -keyalg RSA -keysize 2048 -validity 10000 -alias <YOUR_ALIAS>
```

### 2. Base64 Encode the Keystore
GitHub Actions cannot store binary files easily as secrets. Encode the `.jks` file to Base64.

**Mac/Linux:**
```bash
openssl base64 -in release.jks -out release.jks.base64
```

**Windows (PowerShell):**
```powershell
[Convert]::ToBase64String([IO.File]::ReadAllBytes("./release.jks")) | Out-File -Encoding ascii release.jks.base64
```

### 3. Configure GitHub Secrets
Go to **Settings > Secrets and variables > Actions** in your GitHub repository and add the following secrets:

- `ANDROID_KEY_STORE_FILE_BASE64`: The content of `release.jks.base64`.
- `ANDROID_KEY_ALIAS`: The alias you used in step 1.
- `ANDROID_KEY_PASSWORD`: The password for the key.
- `ANDROID_STORE_PASSWORD`: The password for the keystore (usually the same).

### 4. Enable Signing in `tauri.conf.json` or Environment Variables
Tauri can read these environment variables during the build process to sign the app automatically.

Update `src-tauri/tauri.conf.json` (optional, or rely on env vars):
```json
"bundle": {
  "android": {
    "signingConfig": {
      "storeFile": "release.jks",
      "storePassword": "$ANDROID_STORE_PASSWORD",
      "keyAlias": "$ANDROID_KEY_ALIAS",
      "keyPassword": "$ANDROID_KEY_PASSWORD"
    }
  }
}
```
*Note: It is better to handle this via CI injection rather than committing paths to `tauri.conf.json`.*

### 5. Update GitHub Workflow
Modify `.github/workflows/release.yml` to decode the keystore and inject secrets during the build.

```yaml
      - name: Decode Keystore
        run: |
          echo "${{ secrets.ANDROID_KEY_STORE_FILE_BASE64 }}" | base64 --decode > src-tauri/gen/android/releas.jks

      - name: Build Signed Release
        run: npx tauri android build --apk
        env:
          ANDROID_KEY_ALIAS: ${{ secrets.ANDROID_KEY_ALIAS }}
          ANDROID_KEY_PASSWORD: ${{ secrets.ANDROID_KEY_PASSWORD }}
          ANDROID_STORE_PASSWORD: ${{ secrets.ANDROID_STORE_PASSWORD }}
          ANDROID_KEY_STORE_FILE: ../release.jks
```
