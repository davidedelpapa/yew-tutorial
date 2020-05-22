import livereload from "rollup-plugin-livereload";

export default {
  input: "main.js",
  output: {
    file: "pkg/bundle.js",
    format: "iife",
  },
  plugins: [livereload("pkg")],
};
