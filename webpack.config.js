const path = require("path");
const HtmlPlugin = require("html-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const mode = process.env.NODE_ENV || "development";
const opt =
  mode === "development"
    ? {
        devtool: "inline-source-map",
      }
    : {
        optimization: {
          minimize: true,
          minimizer: [new TerserPlugin({})],
        },
      };

const resolve = {
  extensions: [".tsx", ".ts", ".js", ".wasm"],
};

const tsRule = {
  test: /\.tsx?$/,
  use: "ts-loader",
  exclude: /node_modules/,
};

const browserConfig = {
  ...opt,
  entry: "./www/app.ts",
  resolve,
  module: {
    rules: [
      tsRule,
      {
        test: /\.scss$/,
        use: [
          // fallback to style-loader in development
          "style-loader",
          "css-loader",
          "sass-loader",
        ],
      },
    ],
  },
  output: {
    path: path.resolve("dist"),
    filename: "app.js",
  },
  mode,
  plugins: [
    new CopyPlugin([
      { from: "./www/scenes/*.js", to: "./scenes/[name].js" },
      { from: "./meshes/*.obj", to: "./meshes/[name].obj" },
    ]),
    new WasmPackPlugin({
      crateDirectory: path.resolve("js"),
      outDir: path.resolve("www/lasgun"),
    }),
    new HtmlPlugin({
      template: "./www/index.html",
      title: "lasgun web renderer",
    }),
    new MiniCssExtractPlugin({
      // Options similar to the same options in webpackOptions.output
      // both options are optional
      filename: "[name].css",
      chunkFilename: "[id].css",
    }),
  ],
};

const workerConfig = {
  ...opt,
  entry: "./www/renderer.ts",
  devtool: "inline-source-map",
  resolve,
  module: { rules: [tsRule] },
  target: "webworker",
  output: {
    path: path.resolve("dist"),
    filename: "worker.js",
  },
  mode,
};

module.exports = [browserConfig, workerConfig];
