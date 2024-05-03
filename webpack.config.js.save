const path = require('path');

module.exports = {
  mode: 'development',
  entry: './src/index.js',
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'dist'),
  },
  externals: {
    'cipher-base': 'commonjs cipher-base',
    'hash-base': 'commonjs hash-base',
    'hdkey': 'commonjs hdkey',
    'node-gyp-build': 'commonjs node-gyp-build'
  },
  resolve: {
    fallback: {
      "stream": false,
      "crypto": false,
      "fs": false,
      "os": false
    }
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/preset-env', '@babel/preset-react'],
          },
        },
      },
      {
        test: /\.css$/,
        use: ['style-loader', 'css-loader']
      },
      {
        test: /\.(png|jpg|jpeg|gif)$/i,
        type: 'asset/resource'
      }
    ],
  },
};
