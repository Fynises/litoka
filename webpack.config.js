/* eslint-disable @typescript-eslint/no-var-requires */
/* eslint-disable indent */
const path = require('path');
const nodeExternals = require('webpack-node-externals');

module.exports = [
    {
        name: 'client',
        entry: './client/main.tsx',
        module: {
            rules: [
                {
                    test: /\.tsx?$/,
                    use: 'ts-loader',
                    exclude: /node_modules/,
                },
            ],
        },
        resolve: {
            extensions: ['.tsx', '.ts', '.js'],
        },
        output: {
            filename: '[name].js',
            path: path.resolve(__dirname, 'dist'),
            publicPath: '/dist/'
        },
        optimization: {
            splitChunks: {
                cacheGroups: {
                    vendor: {
                        test: /[\\/]node_modules[\\/](react|react-dom)[\\/]/,
                        name: 'vendor',
                        chunks: 'all'
                    }
                }
            }
        }
    },
    {
        target: 'node',
        externals: [nodeExternals()],
        name: 'server',
        entry: './server/server.ts',
        module: {
            rules: [
                {
                    test: /\.ts?$/,
                    use: 'ts-loader',
                    exclude: /node_modules/,
                },
            ],
        },
        resolve: {
            extensions: ['.ts', '.js'],
        },
        output: {
            filename: 'server-generated.js',
            path: path.resolve(__dirname, 'serverdist'),
        }
    }
];
