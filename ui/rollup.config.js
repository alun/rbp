import node_resolve from '@rollup/plugin-node-resolve';

export default {
  input: 'bundle.js',
  output: {
    dir: 'pkg',
    format: 'iife'
  },
  plugins: [node_resolve()]
};
