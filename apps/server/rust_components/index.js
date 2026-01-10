const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

let nativeBinding = null

try {
  nativeBinding = require('./rust-components.node')
} catch (e) {
  throw new Error(`Failed to load native binding: ${e.message}`)
}

const { add } = nativeBinding

module.exports.add = add
