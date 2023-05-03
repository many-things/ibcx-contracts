/*!
 * @many-things/ibcx-contracts-sdk v0.1.7-rc3
 * (c) frostornge <frostornge@gmail.com>
 * Released under the MIT OR Apache-2.0 License.
 */

(function (global, factory) {
    typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports, require('@cosmjs/encoding'), require('@tanstack/react-query')) :
    typeof define === 'function' && define.amd ? define(['exports', '@cosmjs/encoding', '@tanstack/react-query'], factory) :
    (global = typeof globalThis !== 'undefined' ? globalThis : global || self, factory(global["ibcx-contracts-sdk"] = {}, global.encoding, global.reactQuery));
})(this, (function (exports, encoding, reactQuery) { 'use strict';

    /******************************************************************************
    Copyright (c) Microsoft Corporation.

    Permission to use, copy, modify, and/or distribute this software for any
    purpose with or without fee is hereby granted.

    THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
    REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
    AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
    INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
    LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
    OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
    PERFORMANCE OF THIS SOFTWARE.
    ***************************************************************************** */
    /* global Reflect, Promise */

    var extendStatics = function(d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };

    function __extends(d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    }

    var __assign = function() {
        __assign = Object.assign || function __assign(t) {
            for (var s, i = 1, n = arguments.length; i < n; i++) {
                s = arguments[i];
                for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p)) t[p] = s[p];
            }
            return t;
        };
        return __assign.apply(this, arguments);
    };

    function __awaiter(thisArg, _arguments, P, generator) {
        function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
        return new (P || (P = Promise))(function (resolve, reject) {
            function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
            function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
            function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
            step((generator = generator.apply(thisArg, _arguments || [])).next());
        });
    }

    function __generator(thisArg, body) {
        var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
        return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
        function verb(n) { return function (v) { return step([n, v]); }; }
        function step(op) {
            if (f) throw new TypeError("Generator is already executing.");
            while (g && (g = 0, op[0] && (_ = 0)), _) try {
                if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
                if (y = 0, t) op = [op[0] & 2, t.value];
                switch (op[0]) {
                    case 0: case 1: t = op; break;
                    case 4: _.label++; return { value: op[1], done: false };
                    case 5: _.label++; y = op[1]; op = [0]; continue;
                    case 7: op = _.ops.pop(); _.trys.pop(); continue;
                    default:
                        if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                        if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                        if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                        if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                        if (t[2]) _.ops.pop();
                        _.trys.pop(); continue;
                }
                op = body.call(thisArg, _);
            } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
            if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
        }
    }

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */

    var _0 = /*#__PURE__*/Object.freeze({
        __proto__: null
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var AirdropQueryClient = /** @class */ (function () {
        function AirdropQueryClient(client, contractAddress) {
            var _this = this;
            this.getAirdrop = function (_a) {
                var id = _a.id;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_airdrop: {
                                    id: id
                                }
                            })];
                    });
                });
            };
            this.listAirdrops = function (_a) {
                var option = _a.option;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                list_airdrops: {
                                    option: option
                                }
                            })];
                    });
                });
            };
            this.latestAirdropId = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            latest_airdrop_id: {}
                        })];
                });
            }); };
            this.getClaim = function (_a) {
                var airdrop = _a.airdrop, claimKey = _a.claimKey;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_claim: {
                                    airdrop: airdrop,
                                    claim_key: claimKey
                                }
                            })];
                    });
                });
            };
            this.verifyClaim = function (_a) {
                var claim = _a.claim;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                verify_claim: {
                                    claim: claim
                                }
                            })];
                    });
                });
            };
            this.listClaims = function (_a) {
                var airdrop = _a.airdrop, limit = _a.limit, order = _a.order, startAfter = _a.startAfter;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                list_claims: {
                                    airdrop: airdrop,
                                    limit: limit,
                                    order: order,
                                    start_after: startAfter
                                }
                            })];
                    });
                });
            };
            this.getLabel = function (_a) {
                var label = _a.label;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_label: {
                                    label: label
                                }
                            })];
                    });
                });
            };
            this.listLabels = function (_a) {
                var limit = _a.limit, order = _a.order, startAfter = _a.startAfter;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                list_labels: {
                                    limit: limit,
                                    order: order,
                                    start_after: startAfter
                                }
                            })];
                    });
                });
            };
            this.client = client;
            this.contractAddress = contractAddress;
            this.getAirdrop = this.getAirdrop.bind(this);
            this.listAirdrops = this.listAirdrops.bind(this);
            this.latestAirdropId = this.latestAirdropId.bind(this);
            this.getClaim = this.getClaim.bind(this);
            this.verifyClaim = this.verifyClaim.bind(this);
            this.listClaims = this.listClaims.bind(this);
            this.getLabel = this.getLabel.bind(this);
            this.listLabels = this.listLabels.bind(this);
        }
        return AirdropQueryClient;
    }());
    var AirdropClient = /** @class */ (function (_super) {
        __extends(AirdropClient, _super);
        function AirdropClient(client, sender, contractAddress) {
            var _this = _super.call(this, client, contractAddress) || this;
            _this.register = function (registerPayload, fee, memo, funds) {
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    register: registerPayload
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _a.sent()];
                        }
                    });
                });
            };
            _this.fund = function (airdropId, fee, memo, funds) {
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    fund: airdropId
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _a.sent()];
                        }
                    });
                });
            };
            _this.claim = function (claimPayload, fee, memo, funds) {
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    claim: claimPayload
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _a.sent()];
                        }
                    });
                });
            };
            _this.close = function (airdropId, fee, memo, funds) {
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    close: airdropId
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _a.sent()];
                        }
                    });
                });
            };
            _this.client = client;
            _this.sender = sender;
            _this.contractAddress = contractAddress;
            _this.register = _this.register.bind(_this);
            _this.fund = _this.fund.bind(_this);
            _this.claim = _this.claim.bind(_this);
            _this.close = _this.close.bind(_this);
            return _this;
        }
        return AirdropClient;
    }(AirdropQueryClient));

    var _1 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        AirdropQueryClient: AirdropQueryClient,
        AirdropClient: AirdropClient
    });

    var commonjsGlobal = typeof globalThis !== 'undefined' ? globalThis : typeof window !== 'undefined' ? window : typeof global !== 'undefined' ? global : typeof self !== 'undefined' ? self : {};

    var tx = {};

    var types = {};

    var any = {};

    var indexMinimal = {};

    var minimal$1 = {};

    var aspromise = asPromise;

    /**
     * Callback as used by {@link util.asPromise}.
     * @typedef asPromiseCallback
     * @type {function}
     * @param {Error|null} error Error, if any
     * @param {...*} params Additional arguments
     * @returns {undefined}
     */

    /**
     * Returns a promise from a node-style callback function.
     * @memberof util
     * @param {asPromiseCallback} fn Function to call
     * @param {*} ctx Function context
     * @param {...*} params Function arguments
     * @returns {Promise<*>} Promisified function
     */
    function asPromise(fn, ctx/*, varargs */) {
        var params  = new Array(arguments.length - 1),
            offset  = 0,
            index   = 2,
            pending = true;
        while (index < arguments.length)
            params[offset++] = arguments[index++];
        return new Promise(function executor(resolve, reject) {
            params[offset] = function callback(err/*, varargs */) {
                if (pending) {
                    pending = false;
                    if (err)
                        reject(err);
                    else {
                        var params = new Array(arguments.length - 1),
                            offset = 0;
                        while (offset < params.length)
                            params[offset++] = arguments[offset];
                        resolve.apply(null, params);
                    }
                }
            };
            try {
                fn.apply(ctx || null, params);
            } catch (err) {
                if (pending) {
                    pending = false;
                    reject(err);
                }
            }
        });
    }

    var base64$1 = {};

    (function (exports) {

    	/**
    	 * A minimal base64 implementation for number arrays.
    	 * @memberof util
    	 * @namespace
    	 */
    	var base64 = exports;

    	/**
    	 * Calculates the byte length of a base64 encoded string.
    	 * @param {string} string Base64 encoded string
    	 * @returns {number} Byte length
    	 */
    	base64.length = function length(string) {
    	    var p = string.length;
    	    if (!p)
    	        return 0;
    	    var n = 0;
    	    while (--p % 4 > 1 && string.charAt(p) === "=")
    	        ++n;
    	    return Math.ceil(string.length * 3) / 4 - n;
    	};

    	// Base64 encoding table
    	var b64 = new Array(64);

    	// Base64 decoding table
    	var s64 = new Array(123);

    	// 65..90, 97..122, 48..57, 43, 47
    	for (var i = 0; i < 64;)
    	    s64[b64[i] = i < 26 ? i + 65 : i < 52 ? i + 71 : i < 62 ? i - 4 : i - 59 | 43] = i++;

    	/**
    	 * Encodes a buffer to a base64 encoded string.
    	 * @param {Uint8Array} buffer Source buffer
    	 * @param {number} start Source start
    	 * @param {number} end Source end
    	 * @returns {string} Base64 encoded string
    	 */
    	base64.encode = function encode(buffer, start, end) {
    	    var parts = null,
    	        chunk = [];
    	    var i = 0, // output index
    	        j = 0, // goto index
    	        t;     // temporary
    	    while (start < end) {
    	        var b = buffer[start++];
    	        switch (j) {
    	            case 0:
    	                chunk[i++] = b64[b >> 2];
    	                t = (b & 3) << 4;
    	                j = 1;
    	                break;
    	            case 1:
    	                chunk[i++] = b64[t | b >> 4];
    	                t = (b & 15) << 2;
    	                j = 2;
    	                break;
    	            case 2:
    	                chunk[i++] = b64[t | b >> 6];
    	                chunk[i++] = b64[b & 63];
    	                j = 0;
    	                break;
    	        }
    	        if (i > 8191) {
    	            (parts || (parts = [])).push(String.fromCharCode.apply(String, chunk));
    	            i = 0;
    	        }
    	    }
    	    if (j) {
    	        chunk[i++] = b64[t];
    	        chunk[i++] = 61;
    	        if (j === 1)
    	            chunk[i++] = 61;
    	    }
    	    if (parts) {
    	        if (i)
    	            parts.push(String.fromCharCode.apply(String, chunk.slice(0, i)));
    	        return parts.join("");
    	    }
    	    return String.fromCharCode.apply(String, chunk.slice(0, i));
    	};

    	var invalidEncoding = "invalid encoding";

    	/**
    	 * Decodes a base64 encoded string to a buffer.
    	 * @param {string} string Source string
    	 * @param {Uint8Array} buffer Destination buffer
    	 * @param {number} offset Destination offset
    	 * @returns {number} Number of bytes written
    	 * @throws {Error} If encoding is invalid
    	 */
    	base64.decode = function decode(string, buffer, offset) {
    	    var start = offset;
    	    var j = 0, // goto index
    	        t;     // temporary
    	    for (var i = 0; i < string.length;) {
    	        var c = string.charCodeAt(i++);
    	        if (c === 61 && j > 1)
    	            break;
    	        if ((c = s64[c]) === undefined)
    	            throw Error(invalidEncoding);
    	        switch (j) {
    	            case 0:
    	                t = c;
    	                j = 1;
    	                break;
    	            case 1:
    	                buffer[offset++] = t << 2 | (c & 48) >> 4;
    	                t = c;
    	                j = 2;
    	                break;
    	            case 2:
    	                buffer[offset++] = (t & 15) << 4 | (c & 60) >> 2;
    	                t = c;
    	                j = 3;
    	                break;
    	            case 3:
    	                buffer[offset++] = (t & 3) << 6 | c;
    	                j = 0;
    	                break;
    	        }
    	    }
    	    if (j === 1)
    	        throw Error(invalidEncoding);
    	    return offset - start;
    	};

    	/**
    	 * Tests if the specified string appears to be base64 encoded.
    	 * @param {string} string String to test
    	 * @returns {boolean} `true` if probably base64 encoded, otherwise false
    	 */
    	base64.test = function test(string) {
    	    return /^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$/.test(string);
    	}; 
    } (base64$1));

    var eventemitter = EventEmitter;

    /**
     * Constructs a new event emitter instance.
     * @classdesc A minimal event emitter.
     * @memberof util
     * @constructor
     */
    function EventEmitter() {

        /**
         * Registered listeners.
         * @type {Object.<string,*>}
         * @private
         */
        this._listeners = {};
    }

    /**
     * Registers an event listener.
     * @param {string} evt Event name
     * @param {function} fn Listener
     * @param {*} [ctx] Listener context
     * @returns {util.EventEmitter} `this`
     */
    EventEmitter.prototype.on = function on(evt, fn, ctx) {
        (this._listeners[evt] || (this._listeners[evt] = [])).push({
            fn  : fn,
            ctx : ctx || this
        });
        return this;
    };

    /**
     * Removes an event listener or any matching listeners if arguments are omitted.
     * @param {string} [evt] Event name. Removes all listeners if omitted.
     * @param {function} [fn] Listener to remove. Removes all listeners of `evt` if omitted.
     * @returns {util.EventEmitter} `this`
     */
    EventEmitter.prototype.off = function off(evt, fn) {
        if (evt === undefined)
            this._listeners = {};
        else {
            if (fn === undefined)
                this._listeners[evt] = [];
            else {
                var listeners = this._listeners[evt];
                for (var i = 0; i < listeners.length;)
                    if (listeners[i].fn === fn)
                        listeners.splice(i, 1);
                    else
                        ++i;
            }
        }
        return this;
    };

    /**
     * Emits an event by calling its listeners with the specified arguments.
     * @param {string} evt Event name
     * @param {...*} args Arguments
     * @returns {util.EventEmitter} `this`
     */
    EventEmitter.prototype.emit = function emit(evt) {
        var listeners = this._listeners[evt];
        if (listeners) {
            var args = [],
                i = 1;
            for (; i < arguments.length;)
                args.push(arguments[i++]);
            for (i = 0; i < listeners.length;)
                listeners[i].fn.apply(listeners[i++].ctx, args);
        }
        return this;
    };

    var float = factory(factory);

    /**
     * Reads / writes floats / doubles from / to buffers.
     * @name util.float
     * @namespace
     */

    /**
     * Writes a 32 bit float to a buffer using little endian byte order.
     * @name util.float.writeFloatLE
     * @function
     * @param {number} val Value to write
     * @param {Uint8Array} buf Target buffer
     * @param {number} pos Target buffer offset
     * @returns {undefined}
     */

    /**
     * Writes a 32 bit float to a buffer using big endian byte order.
     * @name util.float.writeFloatBE
     * @function
     * @param {number} val Value to write
     * @param {Uint8Array} buf Target buffer
     * @param {number} pos Target buffer offset
     * @returns {undefined}
     */

    /**
     * Reads a 32 bit float from a buffer using little endian byte order.
     * @name util.float.readFloatLE
     * @function
     * @param {Uint8Array} buf Source buffer
     * @param {number} pos Source buffer offset
     * @returns {number} Value read
     */

    /**
     * Reads a 32 bit float from a buffer using big endian byte order.
     * @name util.float.readFloatBE
     * @function
     * @param {Uint8Array} buf Source buffer
     * @param {number} pos Source buffer offset
     * @returns {number} Value read
     */

    /**
     * Writes a 64 bit double to a buffer using little endian byte order.
     * @name util.float.writeDoubleLE
     * @function
     * @param {number} val Value to write
     * @param {Uint8Array} buf Target buffer
     * @param {number} pos Target buffer offset
     * @returns {undefined}
     */

    /**
     * Writes a 64 bit double to a buffer using big endian byte order.
     * @name util.float.writeDoubleBE
     * @function
     * @param {number} val Value to write
     * @param {Uint8Array} buf Target buffer
     * @param {number} pos Target buffer offset
     * @returns {undefined}
     */

    /**
     * Reads a 64 bit double from a buffer using little endian byte order.
     * @name util.float.readDoubleLE
     * @function
     * @param {Uint8Array} buf Source buffer
     * @param {number} pos Source buffer offset
     * @returns {number} Value read
     */

    /**
     * Reads a 64 bit double from a buffer using big endian byte order.
     * @name util.float.readDoubleBE
     * @function
     * @param {Uint8Array} buf Source buffer
     * @param {number} pos Source buffer offset
     * @returns {number} Value read
     */

    // Factory function for the purpose of node-based testing in modified global environments
    function factory(exports) {

        // float: typed array
        if (typeof Float32Array !== "undefined") (function() {

            var f32 = new Float32Array([ -0 ]),
                f8b = new Uint8Array(f32.buffer),
                le  = f8b[3] === 128;

            function writeFloat_f32_cpy(val, buf, pos) {
                f32[0] = val;
                buf[pos    ] = f8b[0];
                buf[pos + 1] = f8b[1];
                buf[pos + 2] = f8b[2];
                buf[pos + 3] = f8b[3];
            }

            function writeFloat_f32_rev(val, buf, pos) {
                f32[0] = val;
                buf[pos    ] = f8b[3];
                buf[pos + 1] = f8b[2];
                buf[pos + 2] = f8b[1];
                buf[pos + 3] = f8b[0];
            }

            /* istanbul ignore next */
            exports.writeFloatLE = le ? writeFloat_f32_cpy : writeFloat_f32_rev;
            /* istanbul ignore next */
            exports.writeFloatBE = le ? writeFloat_f32_rev : writeFloat_f32_cpy;

            function readFloat_f32_cpy(buf, pos) {
                f8b[0] = buf[pos    ];
                f8b[1] = buf[pos + 1];
                f8b[2] = buf[pos + 2];
                f8b[3] = buf[pos + 3];
                return f32[0];
            }

            function readFloat_f32_rev(buf, pos) {
                f8b[3] = buf[pos    ];
                f8b[2] = buf[pos + 1];
                f8b[1] = buf[pos + 2];
                f8b[0] = buf[pos + 3];
                return f32[0];
            }

            /* istanbul ignore next */
            exports.readFloatLE = le ? readFloat_f32_cpy : readFloat_f32_rev;
            /* istanbul ignore next */
            exports.readFloatBE = le ? readFloat_f32_rev : readFloat_f32_cpy;

        // float: ieee754
        })(); else (function() {

            function writeFloat_ieee754(writeUint, val, buf, pos) {
                var sign = val < 0 ? 1 : 0;
                if (sign)
                    val = -val;
                if (val === 0)
                    writeUint(1 / val > 0 ? /* positive */ 0 : /* negative 0 */ 2147483648, buf, pos);
                else if (isNaN(val))
                    writeUint(2143289344, buf, pos);
                else if (val > 3.4028234663852886e+38) // +-Infinity
                    writeUint((sign << 31 | 2139095040) >>> 0, buf, pos);
                else if (val < 1.1754943508222875e-38) // denormal
                    writeUint((sign << 31 | Math.round(val / 1.401298464324817e-45)) >>> 0, buf, pos);
                else {
                    var exponent = Math.floor(Math.log(val) / Math.LN2),
                        mantissa = Math.round(val * Math.pow(2, -exponent) * 8388608) & 8388607;
                    writeUint((sign << 31 | exponent + 127 << 23 | mantissa) >>> 0, buf, pos);
                }
            }

            exports.writeFloatLE = writeFloat_ieee754.bind(null, writeUintLE);
            exports.writeFloatBE = writeFloat_ieee754.bind(null, writeUintBE);

            function readFloat_ieee754(readUint, buf, pos) {
                var uint = readUint(buf, pos),
                    sign = (uint >> 31) * 2 + 1,
                    exponent = uint >>> 23 & 255,
                    mantissa = uint & 8388607;
                return exponent === 255
                    ? mantissa
                    ? NaN
                    : sign * Infinity
                    : exponent === 0 // denormal
                    ? sign * 1.401298464324817e-45 * mantissa
                    : sign * Math.pow(2, exponent - 150) * (mantissa + 8388608);
            }

            exports.readFloatLE = readFloat_ieee754.bind(null, readUintLE);
            exports.readFloatBE = readFloat_ieee754.bind(null, readUintBE);

        })();

        // double: typed array
        if (typeof Float64Array !== "undefined") (function() {

            var f64 = new Float64Array([-0]),
                f8b = new Uint8Array(f64.buffer),
                le  = f8b[7] === 128;

            function writeDouble_f64_cpy(val, buf, pos) {
                f64[0] = val;
                buf[pos    ] = f8b[0];
                buf[pos + 1] = f8b[1];
                buf[pos + 2] = f8b[2];
                buf[pos + 3] = f8b[3];
                buf[pos + 4] = f8b[4];
                buf[pos + 5] = f8b[5];
                buf[pos + 6] = f8b[6];
                buf[pos + 7] = f8b[7];
            }

            function writeDouble_f64_rev(val, buf, pos) {
                f64[0] = val;
                buf[pos    ] = f8b[7];
                buf[pos + 1] = f8b[6];
                buf[pos + 2] = f8b[5];
                buf[pos + 3] = f8b[4];
                buf[pos + 4] = f8b[3];
                buf[pos + 5] = f8b[2];
                buf[pos + 6] = f8b[1];
                buf[pos + 7] = f8b[0];
            }

            /* istanbul ignore next */
            exports.writeDoubleLE = le ? writeDouble_f64_cpy : writeDouble_f64_rev;
            /* istanbul ignore next */
            exports.writeDoubleBE = le ? writeDouble_f64_rev : writeDouble_f64_cpy;

            function readDouble_f64_cpy(buf, pos) {
                f8b[0] = buf[pos    ];
                f8b[1] = buf[pos + 1];
                f8b[2] = buf[pos + 2];
                f8b[3] = buf[pos + 3];
                f8b[4] = buf[pos + 4];
                f8b[5] = buf[pos + 5];
                f8b[6] = buf[pos + 6];
                f8b[7] = buf[pos + 7];
                return f64[0];
            }

            function readDouble_f64_rev(buf, pos) {
                f8b[7] = buf[pos    ];
                f8b[6] = buf[pos + 1];
                f8b[5] = buf[pos + 2];
                f8b[4] = buf[pos + 3];
                f8b[3] = buf[pos + 4];
                f8b[2] = buf[pos + 5];
                f8b[1] = buf[pos + 6];
                f8b[0] = buf[pos + 7];
                return f64[0];
            }

            /* istanbul ignore next */
            exports.readDoubleLE = le ? readDouble_f64_cpy : readDouble_f64_rev;
            /* istanbul ignore next */
            exports.readDoubleBE = le ? readDouble_f64_rev : readDouble_f64_cpy;

        // double: ieee754
        })(); else (function() {

            function writeDouble_ieee754(writeUint, off0, off1, val, buf, pos) {
                var sign = val < 0 ? 1 : 0;
                if (sign)
                    val = -val;
                if (val === 0) {
                    writeUint(0, buf, pos + off0);
                    writeUint(1 / val > 0 ? /* positive */ 0 : /* negative 0 */ 2147483648, buf, pos + off1);
                } else if (isNaN(val)) {
                    writeUint(0, buf, pos + off0);
                    writeUint(2146959360, buf, pos + off1);
                } else if (val > 1.7976931348623157e+308) { // +-Infinity
                    writeUint(0, buf, pos + off0);
                    writeUint((sign << 31 | 2146435072) >>> 0, buf, pos + off1);
                } else {
                    var mantissa;
                    if (val < 2.2250738585072014e-308) { // denormal
                        mantissa = val / 5e-324;
                        writeUint(mantissa >>> 0, buf, pos + off0);
                        writeUint((sign << 31 | mantissa / 4294967296) >>> 0, buf, pos + off1);
                    } else {
                        var exponent = Math.floor(Math.log(val) / Math.LN2);
                        if (exponent === 1024)
                            exponent = 1023;
                        mantissa = val * Math.pow(2, -exponent);
                        writeUint(mantissa * 4503599627370496 >>> 0, buf, pos + off0);
                        writeUint((sign << 31 | exponent + 1023 << 20 | mantissa * 1048576 & 1048575) >>> 0, buf, pos + off1);
                    }
                }
            }

            exports.writeDoubleLE = writeDouble_ieee754.bind(null, writeUintLE, 0, 4);
            exports.writeDoubleBE = writeDouble_ieee754.bind(null, writeUintBE, 4, 0);

            function readDouble_ieee754(readUint, off0, off1, buf, pos) {
                var lo = readUint(buf, pos + off0),
                    hi = readUint(buf, pos + off1);
                var sign = (hi >> 31) * 2 + 1,
                    exponent = hi >>> 20 & 2047,
                    mantissa = 4294967296 * (hi & 1048575) + lo;
                return exponent === 2047
                    ? mantissa
                    ? NaN
                    : sign * Infinity
                    : exponent === 0 // denormal
                    ? sign * 5e-324 * mantissa
                    : sign * Math.pow(2, exponent - 1075) * (mantissa + 4503599627370496);
            }

            exports.readDoubleLE = readDouble_ieee754.bind(null, readUintLE, 0, 4);
            exports.readDoubleBE = readDouble_ieee754.bind(null, readUintBE, 4, 0);

        })();

        return exports;
    }

    // uint helpers

    function writeUintLE(val, buf, pos) {
        buf[pos    ] =  val        & 255;
        buf[pos + 1] =  val >>> 8  & 255;
        buf[pos + 2] =  val >>> 16 & 255;
        buf[pos + 3] =  val >>> 24;
    }

    function writeUintBE(val, buf, pos) {
        buf[pos    ] =  val >>> 24;
        buf[pos + 1] =  val >>> 16 & 255;
        buf[pos + 2] =  val >>> 8  & 255;
        buf[pos + 3] =  val        & 255;
    }

    function readUintLE(buf, pos) {
        return (buf[pos    ]
              | buf[pos + 1] << 8
              | buf[pos + 2] << 16
              | buf[pos + 3] << 24) >>> 0;
    }

    function readUintBE(buf, pos) {
        return (buf[pos    ] << 24
              | buf[pos + 1] << 16
              | buf[pos + 2] << 8
              | buf[pos + 3]) >>> 0;
    }

    var inquire_1 = inquire;

    /**
     * Requires a module only if available.
     * @memberof util
     * @param {string} moduleName Module to require
     * @returns {?Object} Required module if available and not empty, otherwise `null`
     */
    function inquire(moduleName) {
        try {
            var mod = eval("quire".replace(/^/,"re"))(moduleName); // eslint-disable-line no-eval
            if (mod && (mod.length || Object.keys(mod).length))
                return mod;
        } catch (e) {} // eslint-disable-line no-empty
        return null;
    }

    var utf8$2 = {};

    (function (exports) {

    	/**
    	 * A minimal UTF8 implementation for number arrays.
    	 * @memberof util
    	 * @namespace
    	 */
    	var utf8 = exports;

    	/**
    	 * Calculates the UTF8 byte length of a string.
    	 * @param {string} string String
    	 * @returns {number} Byte length
    	 */
    	utf8.length = function utf8_length(string) {
    	    var len = 0,
    	        c = 0;
    	    for (var i = 0; i < string.length; ++i) {
    	        c = string.charCodeAt(i);
    	        if (c < 128)
    	            len += 1;
    	        else if (c < 2048)
    	            len += 2;
    	        else if ((c & 0xFC00) === 0xD800 && (string.charCodeAt(i + 1) & 0xFC00) === 0xDC00) {
    	            ++i;
    	            len += 4;
    	        } else
    	            len += 3;
    	    }
    	    return len;
    	};

    	/**
    	 * Reads UTF8 bytes as a string.
    	 * @param {Uint8Array} buffer Source buffer
    	 * @param {number} start Source start
    	 * @param {number} end Source end
    	 * @returns {string} String read
    	 */
    	utf8.read = function utf8_read(buffer, start, end) {
    	    var len = end - start;
    	    if (len < 1)
    	        return "";
    	    var parts = null,
    	        chunk = [],
    	        i = 0, // char offset
    	        t;     // temporary
    	    while (start < end) {
    	        t = buffer[start++];
    	        if (t < 128)
    	            chunk[i++] = t;
    	        else if (t > 191 && t < 224)
    	            chunk[i++] = (t & 31) << 6 | buffer[start++] & 63;
    	        else if (t > 239 && t < 365) {
    	            t = ((t & 7) << 18 | (buffer[start++] & 63) << 12 | (buffer[start++] & 63) << 6 | buffer[start++] & 63) - 0x10000;
    	            chunk[i++] = 0xD800 + (t >> 10);
    	            chunk[i++] = 0xDC00 + (t & 1023);
    	        } else
    	            chunk[i++] = (t & 15) << 12 | (buffer[start++] & 63) << 6 | buffer[start++] & 63;
    	        if (i > 8191) {
    	            (parts || (parts = [])).push(String.fromCharCode.apply(String, chunk));
    	            i = 0;
    	        }
    	    }
    	    if (parts) {
    	        if (i)
    	            parts.push(String.fromCharCode.apply(String, chunk.slice(0, i)));
    	        return parts.join("");
    	    }
    	    return String.fromCharCode.apply(String, chunk.slice(0, i));
    	};

    	/**
    	 * Writes a string as UTF8 bytes.
    	 * @param {string} string Source string
    	 * @param {Uint8Array} buffer Destination buffer
    	 * @param {number} offset Destination offset
    	 * @returns {number} Bytes written
    	 */
    	utf8.write = function utf8_write(string, buffer, offset) {
    	    var start = offset,
    	        c1, // character 1
    	        c2; // character 2
    	    for (var i = 0; i < string.length; ++i) {
    	        c1 = string.charCodeAt(i);
    	        if (c1 < 128) {
    	            buffer[offset++] = c1;
    	        } else if (c1 < 2048) {
    	            buffer[offset++] = c1 >> 6       | 192;
    	            buffer[offset++] = c1       & 63 | 128;
    	        } else if ((c1 & 0xFC00) === 0xD800 && ((c2 = string.charCodeAt(i + 1)) & 0xFC00) === 0xDC00) {
    	            c1 = 0x10000 + ((c1 & 0x03FF) << 10) + (c2 & 0x03FF);
    	            ++i;
    	            buffer[offset++] = c1 >> 18      | 240;
    	            buffer[offset++] = c1 >> 12 & 63 | 128;
    	            buffer[offset++] = c1 >> 6  & 63 | 128;
    	            buffer[offset++] = c1       & 63 | 128;
    	        } else {
    	            buffer[offset++] = c1 >> 12      | 224;
    	            buffer[offset++] = c1 >> 6  & 63 | 128;
    	            buffer[offset++] = c1       & 63 | 128;
    	        }
    	    }
    	    return offset - start;
    	}; 
    } (utf8$2));

    var pool_1 = pool;

    /**
     * An allocator as used by {@link util.pool}.
     * @typedef PoolAllocator
     * @type {function}
     * @param {number} size Buffer size
     * @returns {Uint8Array} Buffer
     */

    /**
     * A slicer as used by {@link util.pool}.
     * @typedef PoolSlicer
     * @type {function}
     * @param {number} start Start offset
     * @param {number} end End offset
     * @returns {Uint8Array} Buffer slice
     * @this {Uint8Array}
     */

    /**
     * A general purpose buffer pool.
     * @memberof util
     * @function
     * @param {PoolAllocator} alloc Allocator
     * @param {PoolSlicer} slice Slicer
     * @param {number} [size=8192] Slab size
     * @returns {PoolAllocator} Pooled allocator
     */
    function pool(alloc, slice, size) {
        var SIZE   = size || 8192;
        var MAX    = SIZE >>> 1;
        var slab   = null;
        var offset = SIZE;
        return function pool_alloc(size) {
            if (size < 1 || size > MAX)
                return alloc(size);
            if (offset + size > SIZE) {
                slab = alloc(SIZE);
                offset = 0;
            }
            var buf = slice.call(slab, offset, offset += size);
            if (offset & 7) // align to 32 bit
                offset = (offset | 7) + 1;
            return buf;
        };
    }

    var longbits;
    var hasRequiredLongbits;

    function requireLongbits () {
    	if (hasRequiredLongbits) return longbits;
    	hasRequiredLongbits = 1;
    	longbits = LongBits;

    	var util = requireMinimal();

    	/**
    	 * Constructs new long bits.
    	 * @classdesc Helper class for working with the low and high bits of a 64 bit value.
    	 * @memberof util
    	 * @constructor
    	 * @param {number} lo Low 32 bits, unsigned
    	 * @param {number} hi High 32 bits, unsigned
    	 */
    	function LongBits(lo, hi) {

    	    // note that the casts below are theoretically unnecessary as of today, but older statically
    	    // generated converter code might still call the ctor with signed 32bits. kept for compat.

    	    /**
    	     * Low bits.
    	     * @type {number}
    	     */
    	    this.lo = lo >>> 0;

    	    /**
    	     * High bits.
    	     * @type {number}
    	     */
    	    this.hi = hi >>> 0;
    	}

    	/**
    	 * Zero bits.
    	 * @memberof util.LongBits
    	 * @type {util.LongBits}
    	 */
    	var zero = LongBits.zero = new LongBits(0, 0);

    	zero.toNumber = function() { return 0; };
    	zero.zzEncode = zero.zzDecode = function() { return this; };
    	zero.length = function() { return 1; };

    	/**
    	 * Zero hash.
    	 * @memberof util.LongBits
    	 * @type {string}
    	 */
    	var zeroHash = LongBits.zeroHash = "\0\0\0\0\0\0\0\0";

    	/**
    	 * Constructs new long bits from the specified number.
    	 * @param {number} value Value
    	 * @returns {util.LongBits} Instance
    	 */
    	LongBits.fromNumber = function fromNumber(value) {
    	    if (value === 0)
    	        return zero;
    	    var sign = value < 0;
    	    if (sign)
    	        value = -value;
    	    var lo = value >>> 0,
    	        hi = (value - lo) / 4294967296 >>> 0;
    	    if (sign) {
    	        hi = ~hi >>> 0;
    	        lo = ~lo >>> 0;
    	        if (++lo > 4294967295) {
    	            lo = 0;
    	            if (++hi > 4294967295)
    	                hi = 0;
    	        }
    	    }
    	    return new LongBits(lo, hi);
    	};

    	/**
    	 * Constructs new long bits from a number, long or string.
    	 * @param {Long|number|string} value Value
    	 * @returns {util.LongBits} Instance
    	 */
    	LongBits.from = function from(value) {
    	    if (typeof value === "number")
    	        return LongBits.fromNumber(value);
    	    if (util.isString(value)) {
    	        /* istanbul ignore else */
    	        if (util.Long)
    	            value = util.Long.fromString(value);
    	        else
    	            return LongBits.fromNumber(parseInt(value, 10));
    	    }
    	    return value.low || value.high ? new LongBits(value.low >>> 0, value.high >>> 0) : zero;
    	};

    	/**
    	 * Converts this long bits to a possibly unsafe JavaScript number.
    	 * @param {boolean} [unsigned=false] Whether unsigned or not
    	 * @returns {number} Possibly unsafe number
    	 */
    	LongBits.prototype.toNumber = function toNumber(unsigned) {
    	    if (!unsigned && this.hi >>> 31) {
    	        var lo = ~this.lo + 1 >>> 0,
    	            hi = ~this.hi     >>> 0;
    	        if (!lo)
    	            hi = hi + 1 >>> 0;
    	        return -(lo + hi * 4294967296);
    	    }
    	    return this.lo + this.hi * 4294967296;
    	};

    	/**
    	 * Converts this long bits to a long.
    	 * @param {boolean} [unsigned=false] Whether unsigned or not
    	 * @returns {Long} Long
    	 */
    	LongBits.prototype.toLong = function toLong(unsigned) {
    	    return util.Long
    	        ? new util.Long(this.lo | 0, this.hi | 0, Boolean(unsigned))
    	        /* istanbul ignore next */
    	        : { low: this.lo | 0, high: this.hi | 0, unsigned: Boolean(unsigned) };
    	};

    	var charCodeAt = String.prototype.charCodeAt;

    	/**
    	 * Constructs new long bits from the specified 8 characters long hash.
    	 * @param {string} hash Hash
    	 * @returns {util.LongBits} Bits
    	 */
    	LongBits.fromHash = function fromHash(hash) {
    	    if (hash === zeroHash)
    	        return zero;
    	    return new LongBits(
    	        ( charCodeAt.call(hash, 0)
    	        | charCodeAt.call(hash, 1) << 8
    	        | charCodeAt.call(hash, 2) << 16
    	        | charCodeAt.call(hash, 3) << 24) >>> 0
    	    ,
    	        ( charCodeAt.call(hash, 4)
    	        | charCodeAt.call(hash, 5) << 8
    	        | charCodeAt.call(hash, 6) << 16
    	        | charCodeAt.call(hash, 7) << 24) >>> 0
    	    );
    	};

    	/**
    	 * Converts this long bits to a 8 characters long hash.
    	 * @returns {string} Hash
    	 */
    	LongBits.prototype.toHash = function toHash() {
    	    return String.fromCharCode(
    	        this.lo        & 255,
    	        this.lo >>> 8  & 255,
    	        this.lo >>> 16 & 255,
    	        this.lo >>> 24      ,
    	        this.hi        & 255,
    	        this.hi >>> 8  & 255,
    	        this.hi >>> 16 & 255,
    	        this.hi >>> 24
    	    );
    	};

    	/**
    	 * Zig-zag encodes this long bits.
    	 * @returns {util.LongBits} `this`
    	 */
    	LongBits.prototype.zzEncode = function zzEncode() {
    	    var mask =   this.hi >> 31;
    	    this.hi  = ((this.hi << 1 | this.lo >>> 31) ^ mask) >>> 0;
    	    this.lo  = ( this.lo << 1                   ^ mask) >>> 0;
    	    return this;
    	};

    	/**
    	 * Zig-zag decodes this long bits.
    	 * @returns {util.LongBits} `this`
    	 */
    	LongBits.prototype.zzDecode = function zzDecode() {
    	    var mask = -(this.lo & 1);
    	    this.lo  = ((this.lo >>> 1 | this.hi << 31) ^ mask) >>> 0;
    	    this.hi  = ( this.hi >>> 1                  ^ mask) >>> 0;
    	    return this;
    	};

    	/**
    	 * Calculates the length of this longbits when encoded as a varint.
    	 * @returns {number} Length
    	 */
    	LongBits.prototype.length = function length() {
    	    var part0 =  this.lo,
    	        part1 = (this.lo >>> 28 | this.hi << 4) >>> 0,
    	        part2 =  this.hi >>> 24;
    	    return part2 === 0
    	         ? part1 === 0
    	           ? part0 < 16384
    	             ? part0 < 128 ? 1 : 2
    	             : part0 < 2097152 ? 3 : 4
    	           : part1 < 16384
    	             ? part1 < 128 ? 5 : 6
    	             : part1 < 2097152 ? 7 : 8
    	         : part2 < 128 ? 9 : 10;
    	};
    	return longbits;
    }

    var hasRequiredMinimal;

    function requireMinimal () {
    	if (hasRequiredMinimal) return minimal$1;
    	hasRequiredMinimal = 1;
    	(function (exports) {
    		var util = exports;

    		// used to return a Promise where callback is omitted
    		util.asPromise = aspromise;

    		// converts to / from base64 encoded strings
    		util.base64 = base64$1;

    		// base class of rpc.Service
    		util.EventEmitter = eventemitter;

    		// float handling accross browsers
    		util.float = float;

    		// requires modules optionally and hides the call from bundlers
    		util.inquire = inquire_1;

    		// converts to / from utf8 encoded strings
    		util.utf8 = utf8$2;

    		// provides a node-like buffer pool in the browser
    		util.pool = pool_1;

    		// utility to work with the low and high bits of a 64 bit value
    		util.LongBits = requireLongbits();

    		/**
    		 * Whether running within node or not.
    		 * @memberof util
    		 * @type {boolean}
    		 */
    		util.isNode = Boolean(typeof commonjsGlobal !== "undefined"
    		                   && commonjsGlobal
    		                   && commonjsGlobal.process
    		                   && commonjsGlobal.process.versions
    		                   && commonjsGlobal.process.versions.node);

    		/**
    		 * Global object reference.
    		 * @memberof util
    		 * @type {Object}
    		 */
    		util.global = util.isNode && commonjsGlobal
    		           || typeof window !== "undefined" && window
    		           || typeof self   !== "undefined" && self
    		           || commonjsGlobal; // eslint-disable-line no-invalid-this

    		/**
    		 * An immuable empty array.
    		 * @memberof util
    		 * @type {Array.<*>}
    		 * @const
    		 */
    		util.emptyArray = Object.freeze ? Object.freeze([]) : /* istanbul ignore next */ []; // used on prototypes

    		/**
    		 * An immutable empty object.
    		 * @type {Object}
    		 * @const
    		 */
    		util.emptyObject = Object.freeze ? Object.freeze({}) : /* istanbul ignore next */ {}; // used on prototypes

    		/**
    		 * Tests if the specified value is an integer.
    		 * @function
    		 * @param {*} value Value to test
    		 * @returns {boolean} `true` if the value is an integer
    		 */
    		util.isInteger = Number.isInteger || /* istanbul ignore next */ function isInteger(value) {
    		    return typeof value === "number" && isFinite(value) && Math.floor(value) === value;
    		};

    		/**
    		 * Tests if the specified value is a string.
    		 * @param {*} value Value to test
    		 * @returns {boolean} `true` if the value is a string
    		 */
    		util.isString = function isString(value) {
    		    return typeof value === "string" || value instanceof String;
    		};

    		/**
    		 * Tests if the specified value is a non-null object.
    		 * @param {*} value Value to test
    		 * @returns {boolean} `true` if the value is a non-null object
    		 */
    		util.isObject = function isObject(value) {
    		    return value && typeof value === "object";
    		};

    		/**
    		 * Checks if a property on a message is considered to be present.
    		 * This is an alias of {@link util.isSet}.
    		 * @function
    		 * @param {Object} obj Plain object or message instance
    		 * @param {string} prop Property name
    		 * @returns {boolean} `true` if considered to be present, otherwise `false`
    		 */
    		util.isset =

    		/**
    		 * Checks if a property on a message is considered to be present.
    		 * @param {Object} obj Plain object or message instance
    		 * @param {string} prop Property name
    		 * @returns {boolean} `true` if considered to be present, otherwise `false`
    		 */
    		util.isSet = function isSet(obj, prop) {
    		    var value = obj[prop];
    		    if (value != null && obj.hasOwnProperty(prop)) // eslint-disable-line eqeqeq, no-prototype-builtins
    		        return typeof value !== "object" || (Array.isArray(value) ? value.length : Object.keys(value).length) > 0;
    		    return false;
    		};

    		/**
    		 * Any compatible Buffer instance.
    		 * This is a minimal stand-alone definition of a Buffer instance. The actual type is that exported by node's typings.
    		 * @interface Buffer
    		 * @extends Uint8Array
    		 */

    		/**
    		 * Node's Buffer class if available.
    		 * @type {Constructor<Buffer>}
    		 */
    		util.Buffer = (function() {
    		    try {
    		        var Buffer = util.inquire("buffer").Buffer;
    		        // refuse to use non-node buffers if not explicitly assigned (perf reasons):
    		        return Buffer.prototype.utf8Write ? Buffer : /* istanbul ignore next */ null;
    		    } catch (e) {
    		        /* istanbul ignore next */
    		        return null;
    		    }
    		})();

    		// Internal alias of or polyfull for Buffer.from.
    		util._Buffer_from = null;

    		// Internal alias of or polyfill for Buffer.allocUnsafe.
    		util._Buffer_allocUnsafe = null;

    		/**
    		 * Creates a new buffer of whatever type supported by the environment.
    		 * @param {number|number[]} [sizeOrArray=0] Buffer size or number array
    		 * @returns {Uint8Array|Buffer} Buffer
    		 */
    		util.newBuffer = function newBuffer(sizeOrArray) {
    		    /* istanbul ignore next */
    		    return typeof sizeOrArray === "number"
    		        ? util.Buffer
    		            ? util._Buffer_allocUnsafe(sizeOrArray)
    		            : new util.Array(sizeOrArray)
    		        : util.Buffer
    		            ? util._Buffer_from(sizeOrArray)
    		            : typeof Uint8Array === "undefined"
    		                ? sizeOrArray
    		                : new Uint8Array(sizeOrArray);
    		};

    		/**
    		 * Array implementation used in the browser. `Uint8Array` if supported, otherwise `Array`.
    		 * @type {Constructor<Uint8Array>}
    		 */
    		util.Array = typeof Uint8Array !== "undefined" ? Uint8Array /* istanbul ignore next */ : Array;

    		/**
    		 * Any compatible Long instance.
    		 * This is a minimal stand-alone definition of a Long instance. The actual type is that exported by long.js.
    		 * @interface Long
    		 * @property {number} low Low bits
    		 * @property {number} high High bits
    		 * @property {boolean} unsigned Whether unsigned or not
    		 */

    		/**
    		 * Long.js's Long class if available.
    		 * @type {Constructor<Long>}
    		 */
    		util.Long = /* istanbul ignore next */ util.global.dcodeIO && /* istanbul ignore next */ util.global.dcodeIO.Long
    		         || /* istanbul ignore next */ util.global.Long
    		         || util.inquire("long");

    		/**
    		 * Regular expression used to verify 2 bit (`bool`) map keys.
    		 * @type {RegExp}
    		 * @const
    		 */
    		util.key2Re = /^true|false|0|1$/;

    		/**
    		 * Regular expression used to verify 32 bit (`int32` etc.) map keys.
    		 * @type {RegExp}
    		 * @const
    		 */
    		util.key32Re = /^-?(?:0|[1-9][0-9]*)$/;

    		/**
    		 * Regular expression used to verify 64 bit (`int64` etc.) map keys.
    		 * @type {RegExp}
    		 * @const
    		 */
    		util.key64Re = /^(?:[\\x00-\\xff]{8}|-?(?:0|[1-9][0-9]*))$/;

    		/**
    		 * Converts a number or long to an 8 characters long hash string.
    		 * @param {Long|number} value Value to convert
    		 * @returns {string} Hash
    		 */
    		util.longToHash = function longToHash(value) {
    		    return value
    		        ? util.LongBits.from(value).toHash()
    		        : util.LongBits.zeroHash;
    		};

    		/**
    		 * Converts an 8 characters long hash string to a long or number.
    		 * @param {string} hash Hash
    		 * @param {boolean} [unsigned=false] Whether unsigned or not
    		 * @returns {Long|number} Original value
    		 */
    		util.longFromHash = function longFromHash(hash, unsigned) {
    		    var bits = util.LongBits.fromHash(hash);
    		    if (util.Long)
    		        return util.Long.fromBits(bits.lo, bits.hi, unsigned);
    		    return bits.toNumber(Boolean(unsigned));
    		};

    		/**
    		 * Merges the properties of the source object into the destination object.
    		 * @memberof util
    		 * @param {Object.<string,*>} dst Destination object
    		 * @param {Object.<string,*>} src Source object
    		 * @param {boolean} [ifNotSet=false] Merges only if the key is not already set
    		 * @returns {Object.<string,*>} Destination object
    		 */
    		function merge(dst, src, ifNotSet) { // used by converters
    		    for (var keys = Object.keys(src), i = 0; i < keys.length; ++i)
    		        if (dst[keys[i]] === undefined || !ifNotSet)
    		            dst[keys[i]] = src[keys[i]];
    		    return dst;
    		}

    		util.merge = merge;

    		/**
    		 * Converts the first character of a string to lower case.
    		 * @param {string} str String to convert
    		 * @returns {string} Converted string
    		 */
    		util.lcFirst = function lcFirst(str) {
    		    return str.charAt(0).toLowerCase() + str.substring(1);
    		};

    		/**
    		 * Creates a custom error constructor.
    		 * @memberof util
    		 * @param {string} name Error name
    		 * @returns {Constructor<Error>} Custom error constructor
    		 */
    		function newError(name) {

    		    function CustomError(message, properties) {

    		        if (!(this instanceof CustomError))
    		            return new CustomError(message, properties);

    		        // Error.call(this, message);
    		        // ^ just returns a new error instance because the ctor can be called as a function

    		        Object.defineProperty(this, "message", { get: function() { return message; } });

    		        /* istanbul ignore next */
    		        if (Error.captureStackTrace) // node
    		            Error.captureStackTrace(this, CustomError);
    		        else
    		            Object.defineProperty(this, "stack", { value: new Error().stack || "" });

    		        if (properties)
    		            merge(this, properties);
    		    }

    		    (CustomError.prototype = Object.create(Error.prototype)).constructor = CustomError;

    		    Object.defineProperty(CustomError.prototype, "name", { get: function() { return name; } });

    		    CustomError.prototype.toString = function toString() {
    		        return this.name + ": " + this.message;
    		    };

    		    return CustomError;
    		}

    		util.newError = newError;

    		/**
    		 * Constructs a new protocol error.
    		 * @classdesc Error subclass indicating a protocol specifc error.
    		 * @memberof util
    		 * @extends Error
    		 * @template T extends Message<T>
    		 * @constructor
    		 * @param {string} message Error message
    		 * @param {Object.<string,*>} [properties] Additional properties
    		 * @example
    		 * try {
    		 *     MyMessage.decode(someBuffer); // throws if required fields are missing
    		 * } catch (e) {
    		 *     if (e instanceof ProtocolError && e.instance)
    		 *         console.log("decoded so far: " + JSON.stringify(e.instance));
    		 * }
    		 */
    		util.ProtocolError = newError("ProtocolError");

    		/**
    		 * So far decoded message instance.
    		 * @name util.ProtocolError#instance
    		 * @type {Message<T>}
    		 */

    		/**
    		 * A OneOf getter as returned by {@link util.oneOfGetter}.
    		 * @typedef OneOfGetter
    		 * @type {function}
    		 * @returns {string|undefined} Set field name, if any
    		 */

    		/**
    		 * Builds a getter for a oneof's present field name.
    		 * @param {string[]} fieldNames Field names
    		 * @returns {OneOfGetter} Unbound getter
    		 */
    		util.oneOfGetter = function getOneOf(fieldNames) {
    		    var fieldMap = {};
    		    for (var i = 0; i < fieldNames.length; ++i)
    		        fieldMap[fieldNames[i]] = 1;

    		    /**
    		     * @returns {string|undefined} Set field name, if any
    		     * @this Object
    		     * @ignore
    		     */
    		    return function() { // eslint-disable-line consistent-return
    		        for (var keys = Object.keys(this), i = keys.length - 1; i > -1; --i)
    		            if (fieldMap[keys[i]] === 1 && this[keys[i]] !== undefined && this[keys[i]] !== null)
    		                return keys[i];
    		    };
    		};

    		/**
    		 * A OneOf setter as returned by {@link util.oneOfSetter}.
    		 * @typedef OneOfSetter
    		 * @type {function}
    		 * @param {string|undefined} value Field name
    		 * @returns {undefined}
    		 */

    		/**
    		 * Builds a setter for a oneof's present field name.
    		 * @param {string[]} fieldNames Field names
    		 * @returns {OneOfSetter} Unbound setter
    		 */
    		util.oneOfSetter = function setOneOf(fieldNames) {

    		    /**
    		     * @param {string} name Field name
    		     * @returns {undefined}
    		     * @this Object
    		     * @ignore
    		     */
    		    return function(name) {
    		        for (var i = 0; i < fieldNames.length; ++i)
    		            if (fieldNames[i] !== name)
    		                delete this[fieldNames[i]];
    		    };
    		};

    		/**
    		 * Default conversion options used for {@link Message#toJSON} implementations.
    		 *
    		 * These options are close to proto3's JSON mapping with the exception that internal types like Any are handled just like messages. More precisely:
    		 *
    		 * - Longs become strings
    		 * - Enums become string keys
    		 * - Bytes become base64 encoded strings
    		 * - (Sub-)Messages become plain objects
    		 * - Maps become plain objects with all string keys
    		 * - Repeated fields become arrays
    		 * - NaN and Infinity for float and double fields become strings
    		 *
    		 * @type {IConversionOptions}
    		 * @see https://developers.google.com/protocol-buffers/docs/proto3?hl=en#json
    		 */
    		util.toJSONOptions = {
    		    longs: String,
    		    enums: String,
    		    bytes: String,
    		    json: true
    		};

    		// Sets up buffer utility according to the environment (called in index-minimal)
    		util._configure = function() {
    		    var Buffer = util.Buffer;
    		    /* istanbul ignore if */
    		    if (!Buffer) {
    		        util._Buffer_from = util._Buffer_allocUnsafe = null;
    		        return;
    		    }
    		    // because node 4.x buffers are incompatible & immutable
    		    // see: https://github.com/dcodeIO/protobuf.js/pull/665
    		    util._Buffer_from = Buffer.from !== Uint8Array.from && Buffer.from ||
    		        /* istanbul ignore next */
    		        function Buffer_from(value, encoding) {
    		            return new Buffer(value, encoding);
    		        };
    		    util._Buffer_allocUnsafe = Buffer.allocUnsafe ||
    		        /* istanbul ignore next */
    		        function Buffer_allocUnsafe(size) {
    		            return new Buffer(size);
    		        };
    		}; 
    	} (minimal$1));
    	return minimal$1;
    }

    var writer = Writer$1;

    var util$4      = requireMinimal();

    var BufferWriter$1; // cyclic

    var LongBits$1  = util$4.LongBits,
        base64    = util$4.base64,
        utf8$1      = util$4.utf8;

    /**
     * Constructs a new writer operation instance.
     * @classdesc Scheduled writer operation.
     * @constructor
     * @param {function(*, Uint8Array, number)} fn Function to call
     * @param {number} len Value byte length
     * @param {*} val Value to write
     * @ignore
     */
    function Op(fn, len, val) {

        /**
         * Function to call.
         * @type {function(Uint8Array, number, *)}
         */
        this.fn = fn;

        /**
         * Value byte length.
         * @type {number}
         */
        this.len = len;

        /**
         * Next operation.
         * @type {Writer.Op|undefined}
         */
        this.next = undefined;

        /**
         * Value to write.
         * @type {*}
         */
        this.val = val; // type varies
    }

    /* istanbul ignore next */
    function noop() {} // eslint-disable-line no-empty-function

    /**
     * Constructs a new writer state instance.
     * @classdesc Copied writer state.
     * @memberof Writer
     * @constructor
     * @param {Writer} writer Writer to copy state from
     * @ignore
     */
    function State(writer) {

        /**
         * Current head.
         * @type {Writer.Op}
         */
        this.head = writer.head;

        /**
         * Current tail.
         * @type {Writer.Op}
         */
        this.tail = writer.tail;

        /**
         * Current buffer length.
         * @type {number}
         */
        this.len = writer.len;

        /**
         * Next state.
         * @type {State|null}
         */
        this.next = writer.states;
    }

    /**
     * Constructs a new writer instance.
     * @classdesc Wire format writer using `Uint8Array` if available, otherwise `Array`.
     * @constructor
     */
    function Writer$1() {

        /**
         * Current length.
         * @type {number}
         */
        this.len = 0;

        /**
         * Operations head.
         * @type {Object}
         */
        this.head = new Op(noop, 0, 0);

        /**
         * Operations tail
         * @type {Object}
         */
        this.tail = this.head;

        /**
         * Linked forked states.
         * @type {Object|null}
         */
        this.states = null;

        // When a value is written, the writer calculates its byte length and puts it into a linked
        // list of operations to perform when finish() is called. This both allows us to allocate
        // buffers of the exact required size and reduces the amount of work we have to do compared
        // to first calculating over objects and then encoding over objects. In our case, the encoding
        // part is just a linked list walk calling operations with already prepared values.
    }

    var create$1 = function create() {
        return util$4.Buffer
            ? function create_buffer_setup() {
                return (Writer$1.create = function create_buffer() {
                    return new BufferWriter$1();
                })();
            }
            /* istanbul ignore next */
            : function create_array() {
                return new Writer$1();
            };
    };

    /**
     * Creates a new writer.
     * @function
     * @returns {BufferWriter|Writer} A {@link BufferWriter} when Buffers are supported, otherwise a {@link Writer}
     */
    Writer$1.create = create$1();

    /**
     * Allocates a buffer of the specified size.
     * @param {number} size Buffer size
     * @returns {Uint8Array} Buffer
     */
    Writer$1.alloc = function alloc(size) {
        return new util$4.Array(size);
    };

    // Use Uint8Array buffer pool in the browser, just like node does with buffers
    /* istanbul ignore else */
    if (util$4.Array !== Array)
        Writer$1.alloc = util$4.pool(Writer$1.alloc, util$4.Array.prototype.subarray);

    /**
     * Pushes a new operation to the queue.
     * @param {function(Uint8Array, number, *)} fn Function to call
     * @param {number} len Value byte length
     * @param {number} val Value to write
     * @returns {Writer} `this`
     * @private
     */
    Writer$1.prototype._push = function push(fn, len, val) {
        this.tail = this.tail.next = new Op(fn, len, val);
        this.len += len;
        return this;
    };

    function writeByte(val, buf, pos) {
        buf[pos] = val & 255;
    }

    function writeVarint32(val, buf, pos) {
        while (val > 127) {
            buf[pos++] = val & 127 | 128;
            val >>>= 7;
        }
        buf[pos] = val;
    }

    /**
     * Constructs a new varint writer operation instance.
     * @classdesc Scheduled varint writer operation.
     * @extends Op
     * @constructor
     * @param {number} len Value byte length
     * @param {number} val Value to write
     * @ignore
     */
    function VarintOp(len, val) {
        this.len = len;
        this.next = undefined;
        this.val = val;
    }

    VarintOp.prototype = Object.create(Op.prototype);
    VarintOp.prototype.fn = writeVarint32;

    /**
     * Writes an unsigned 32 bit value as a varint.
     * @param {number} value Value to write
     * @returns {Writer} `this`
     */
    Writer$1.prototype.uint32 = function write_uint32(value) {
        // here, the call to this.push has been inlined and a varint specific Op subclass is used.
        // uint32 is by far the most frequently used operation and benefits significantly from this.
        this.len += (this.tail = this.tail.next = new VarintOp(
            (value = value >>> 0)
                    < 128       ? 1
            : value < 16384     ? 2
            : value < 2097152   ? 3
            : value < 268435456 ? 4
            :                     5,
        value)).len;
        return this;
    };

    /**
     * Writes a signed 32 bit value as a varint.
     * @function
     * @param {number} value Value to write
     * @returns {Writer} `this`
     */
    Writer$1.prototype.int32 = function write_int32(value) {
        return value < 0
            ? this._push(writeVarint64, 10, LongBits$1.fromNumber(value)) // 10 bytes per spec
            : this.uint32(value);
    };

    /**
     * Writes a 32 bit value as a varint, zig-zag encoded.
     * @param {number} value Value to write
     * @returns {Writer} `this`
     */
    Writer$1.prototype.sint32 = function write_sint32(value) {
        return this.uint32((value << 1 ^ value >> 31) >>> 0);
    };

    function writeVarint64(val, buf, pos) {
        while (val.hi) {
            buf[pos++] = val.lo & 127 | 128;
            val.lo = (val.lo >>> 7 | val.hi << 25) >>> 0;
            val.hi >>>= 7;
        }
        while (val.lo > 127) {
            buf[pos++] = val.lo & 127 | 128;
            val.lo = val.lo >>> 7;
        }
        buf[pos++] = val.lo;
    }

    /**
     * Writes an unsigned 64 bit value as a varint.
     * @param {Long|number|string} value Value to write
     * @returns {Writer} `this`
     * @throws {TypeError} If `value` is a string and no long library is present.
     */
    Writer$1.prototype.uint64 = function write_uint64(value) {
        var bits = LongBits$1.from(value);
        return this._push(writeVarint64, bits.length(), bits);
    };

    /**
     * Writes a signed 64 bit value as a varint.
     * @function
     * @param {Long|number|string} value Value to write
     * @returns {Writer} `this`
     * @throws {TypeError} If `value` is a string and no long library is present.
     */
    Writer$1.prototype.int64 = Writer$1.prototype.uint64;

    /**
     * Writes a signed 64 bit value as a varint, zig-zag encoded.
     * @param {Long|number|string} value Value to write
     * @returns {Writer} `this`
     * @throws {TypeError} If `value` is a string and no long library is present.
     */
    Writer$1.prototype.sint64 = function write_sint64(value) {
        var bits = LongBits$1.from(value).zzEncode();
        return this._push(writeVarint64, bits.length(), bits);
    };

    /**
     * Writes a boolish value as a varint.
     * @param {boolean} value Value to write
     * @returns {Writer} `this`
     */
    Writer$1.prototype.bool = function write_bool(value) {
        return this._push(writeByte, 1, value ? 1 : 0);
    };

    function writeFixed32(val, buf, pos) {
        buf[pos    ] =  val         & 255;
        buf[pos + 1] =  val >>> 8   & 255;
        buf[pos + 2] =  val >>> 16  & 255;
        buf[pos + 3] =  val >>> 24;
    }

    /**
     * Writes an unsigned 32 bit value as fixed 32 bits.
     * @param {number} value Value to write
     * @returns {Writer} `this`
     */
    Writer$1.prototype.fixed32 = function write_fixed32(value) {
        return this._push(writeFixed32, 4, value >>> 0);
    };

    /**
     * Writes a signed 32 bit value as fixed 32 bits.
     * @function
     * @param {number} value Value to write
     * @returns {Writer} `this`
     */
    Writer$1.prototype.sfixed32 = Writer$1.prototype.fixed32;

    /**
     * Writes an unsigned 64 bit value as fixed 64 bits.
     * @param {Long|number|string} value Value to write
     * @returns {Writer} `this`
     * @throws {TypeError} If `value` is a string and no long library is present.
     */
    Writer$1.prototype.fixed64 = function write_fixed64(value) {
        var bits = LongBits$1.from(value);
        return this._push(writeFixed32, 4, bits.lo)._push(writeFixed32, 4, bits.hi);
    };

    /**
     * Writes a signed 64 bit value as fixed 64 bits.
     * @function
     * @param {Long|number|string} value Value to write
     * @returns {Writer} `this`
     * @throws {TypeError} If `value` is a string and no long library is present.
     */
    Writer$1.prototype.sfixed64 = Writer$1.prototype.fixed64;

    /**
     * Writes a float (32 bit).
     * @function
     * @param {number} value Value to write
     * @returns {Writer} `this`
     */
    Writer$1.prototype.float = function write_float(value) {
        return this._push(util$4.float.writeFloatLE, 4, value);
    };

    /**
     * Writes a double (64 bit float).
     * @function
     * @param {number} value Value to write
     * @returns {Writer} `this`
     */
    Writer$1.prototype.double = function write_double(value) {
        return this._push(util$4.float.writeDoubleLE, 8, value);
    };

    var writeBytes = util$4.Array.prototype.set
        ? function writeBytes_set(val, buf, pos) {
            buf.set(val, pos); // also works for plain array values
        }
        /* istanbul ignore next */
        : function writeBytes_for(val, buf, pos) {
            for (var i = 0; i < val.length; ++i)
                buf[pos + i] = val[i];
        };

    /**
     * Writes a sequence of bytes.
     * @param {Uint8Array|string} value Buffer or base64 encoded string to write
     * @returns {Writer} `this`
     */
    Writer$1.prototype.bytes = function write_bytes(value) {
        var len = value.length >>> 0;
        if (!len)
            return this._push(writeByte, 1, 0);
        if (util$4.isString(value)) {
            var buf = Writer$1.alloc(len = base64.length(value));
            base64.decode(value, buf, 0);
            value = buf;
        }
        return this.uint32(len)._push(writeBytes, len, value);
    };

    /**
     * Writes a string.
     * @param {string} value Value to write
     * @returns {Writer} `this`
     */
    Writer$1.prototype.string = function write_string(value) {
        var len = utf8$1.length(value);
        return len
            ? this.uint32(len)._push(utf8$1.write, len, value)
            : this._push(writeByte, 1, 0);
    };

    /**
     * Forks this writer's state by pushing it to a stack.
     * Calling {@link Writer#reset|reset} or {@link Writer#ldelim|ldelim} resets the writer to the previous state.
     * @returns {Writer} `this`
     */
    Writer$1.prototype.fork = function fork() {
        this.states = new State(this);
        this.head = this.tail = new Op(noop, 0, 0);
        this.len = 0;
        return this;
    };

    /**
     * Resets this instance to the last state.
     * @returns {Writer} `this`
     */
    Writer$1.prototype.reset = function reset() {
        if (this.states) {
            this.head   = this.states.head;
            this.tail   = this.states.tail;
            this.len    = this.states.len;
            this.states = this.states.next;
        } else {
            this.head = this.tail = new Op(noop, 0, 0);
            this.len  = 0;
        }
        return this;
    };

    /**
     * Resets to the last state and appends the fork state's current write length as a varint followed by its operations.
     * @returns {Writer} `this`
     */
    Writer$1.prototype.ldelim = function ldelim() {
        var head = this.head,
            tail = this.tail,
            len  = this.len;
        this.reset().uint32(len);
        if (len) {
            this.tail.next = head.next; // skip noop
            this.tail = tail;
            this.len += len;
        }
        return this;
    };

    /**
     * Finishes the write operation.
     * @returns {Uint8Array} Finished buffer
     */
    Writer$1.prototype.finish = function finish() {
        var head = this.head.next, // skip noop
            buf  = this.constructor.alloc(this.len),
            pos  = 0;
        while (head) {
            head.fn(head.val, buf, pos);
            pos += head.len;
            head = head.next;
        }
        // this.head = this.tail = null;
        return buf;
    };

    Writer$1._configure = function(BufferWriter_) {
        BufferWriter$1 = BufferWriter_;
        Writer$1.create = create$1();
        BufferWriter$1._configure();
    };

    var writer_buffer = BufferWriter;

    // extends Writer
    var Writer = writer;
    (BufferWriter.prototype = Object.create(Writer.prototype)).constructor = BufferWriter;

    var util$3 = requireMinimal();

    /**
     * Constructs a new buffer writer instance.
     * @classdesc Wire format writer using node buffers.
     * @extends Writer
     * @constructor
     */
    function BufferWriter() {
        Writer.call(this);
    }

    BufferWriter._configure = function () {
        /**
         * Allocates a buffer of the specified size.
         * @function
         * @param {number} size Buffer size
         * @returns {Buffer} Buffer
         */
        BufferWriter.alloc = util$3._Buffer_allocUnsafe;

        BufferWriter.writeBytesBuffer = util$3.Buffer && util$3.Buffer.prototype instanceof Uint8Array && util$3.Buffer.prototype.set.name === "set"
            ? function writeBytesBuffer_set(val, buf, pos) {
              buf.set(val, pos); // faster than copy (requires node >= 4 where Buffers extend Uint8Array and set is properly inherited)
              // also works for plain array values
            }
            /* istanbul ignore next */
            : function writeBytesBuffer_copy(val, buf, pos) {
              if (val.copy) // Buffer values
                val.copy(buf, pos, 0, val.length);
              else for (var i = 0; i < val.length;) // plain array values
                buf[pos++] = val[i++];
            };
    };


    /**
     * @override
     */
    BufferWriter.prototype.bytes = function write_bytes_buffer(value) {
        if (util$3.isString(value))
            value = util$3._Buffer_from(value, "base64");
        var len = value.length >>> 0;
        this.uint32(len);
        if (len)
            this._push(BufferWriter.writeBytesBuffer, len, value);
        return this;
    };

    function writeStringBuffer(val, buf, pos) {
        if (val.length < 40) // plain js is faster for short strings (probably due to redundant assertions)
            util$3.utf8.write(val, buf, pos);
        else if (buf.utf8Write)
            buf.utf8Write(val, pos);
        else
            buf.write(val, pos);
    }

    /**
     * @override
     */
    BufferWriter.prototype.string = function write_string_buffer(value) {
        var len = util$3.Buffer.byteLength(value);
        this.uint32(len);
        if (len)
            this._push(writeStringBuffer, len, value);
        return this;
    };


    /**
     * Finishes the write operation.
     * @name BufferWriter#finish
     * @function
     * @returns {Buffer} Finished buffer
     */

    BufferWriter._configure();

    var reader = Reader$1;

    var util$2      = requireMinimal();

    var BufferReader$1; // cyclic

    var LongBits  = util$2.LongBits,
        utf8      = util$2.utf8;

    /* istanbul ignore next */
    function indexOutOfRange(reader, writeLength) {
        return RangeError("index out of range: " + reader.pos + " + " + (writeLength || 1) + " > " + reader.len);
    }

    /**
     * Constructs a new reader instance using the specified buffer.
     * @classdesc Wire format reader using `Uint8Array` if available, otherwise `Array`.
     * @constructor
     * @param {Uint8Array} buffer Buffer to read from
     */
    function Reader$1(buffer) {

        /**
         * Read buffer.
         * @type {Uint8Array}
         */
        this.buf = buffer;

        /**
         * Read buffer position.
         * @type {number}
         */
        this.pos = 0;

        /**
         * Read buffer length.
         * @type {number}
         */
        this.len = buffer.length;
    }

    var create_array = typeof Uint8Array !== "undefined"
        ? function create_typed_array(buffer) {
            if (buffer instanceof Uint8Array || Array.isArray(buffer))
                return new Reader$1(buffer);
            throw Error("illegal buffer");
        }
        /* istanbul ignore next */
        : function create_array(buffer) {
            if (Array.isArray(buffer))
                return new Reader$1(buffer);
            throw Error("illegal buffer");
        };

    var create = function create() {
        return util$2.Buffer
            ? function create_buffer_setup(buffer) {
                return (Reader$1.create = function create_buffer(buffer) {
                    return util$2.Buffer.isBuffer(buffer)
                        ? new BufferReader$1(buffer)
                        /* istanbul ignore next */
                        : create_array(buffer);
                })(buffer);
            }
            /* istanbul ignore next */
            : create_array;
    };

    /**
     * Creates a new reader using the specified buffer.
     * @function
     * @param {Uint8Array|Buffer} buffer Buffer to read from
     * @returns {Reader|BufferReader} A {@link BufferReader} if `buffer` is a Buffer, otherwise a {@link Reader}
     * @throws {Error} If `buffer` is not a valid buffer
     */
    Reader$1.create = create();

    Reader$1.prototype._slice = util$2.Array.prototype.subarray || /* istanbul ignore next */ util$2.Array.prototype.slice;

    /**
     * Reads a varint as an unsigned 32 bit value.
     * @function
     * @returns {number} Value read
     */
    Reader$1.prototype.uint32 = (function read_uint32_setup() {
        var value = 4294967295; // optimizer type-hint, tends to deopt otherwise (?!)
        return function read_uint32() {
            value = (         this.buf[this.pos] & 127       ) >>> 0; if (this.buf[this.pos++] < 128) return value;
            value = (value | (this.buf[this.pos] & 127) <<  7) >>> 0; if (this.buf[this.pos++] < 128) return value;
            value = (value | (this.buf[this.pos] & 127) << 14) >>> 0; if (this.buf[this.pos++] < 128) return value;
            value = (value | (this.buf[this.pos] & 127) << 21) >>> 0; if (this.buf[this.pos++] < 128) return value;
            value = (value | (this.buf[this.pos] &  15) << 28) >>> 0; if (this.buf[this.pos++] < 128) return value;

            /* istanbul ignore if */
            if ((this.pos += 5) > this.len) {
                this.pos = this.len;
                throw indexOutOfRange(this, 10);
            }
            return value;
        };
    })();

    /**
     * Reads a varint as a signed 32 bit value.
     * @returns {number} Value read
     */
    Reader$1.prototype.int32 = function read_int32() {
        return this.uint32() | 0;
    };

    /**
     * Reads a zig-zag encoded varint as a signed 32 bit value.
     * @returns {number} Value read
     */
    Reader$1.prototype.sint32 = function read_sint32() {
        var value = this.uint32();
        return value >>> 1 ^ -(value & 1) | 0;
    };

    /* eslint-disable no-invalid-this */

    function readLongVarint() {
        // tends to deopt with local vars for octet etc.
        var bits = new LongBits(0, 0);
        var i = 0;
        if (this.len - this.pos > 4) { // fast route (lo)
            for (; i < 4; ++i) {
                // 1st..4th
                bits.lo = (bits.lo | (this.buf[this.pos] & 127) << i * 7) >>> 0;
                if (this.buf[this.pos++] < 128)
                    return bits;
            }
            // 5th
            bits.lo = (bits.lo | (this.buf[this.pos] & 127) << 28) >>> 0;
            bits.hi = (bits.hi | (this.buf[this.pos] & 127) >>  4) >>> 0;
            if (this.buf[this.pos++] < 128)
                return bits;
            i = 0;
        } else {
            for (; i < 3; ++i) {
                /* istanbul ignore if */
                if (this.pos >= this.len)
                    throw indexOutOfRange(this);
                // 1st..3th
                bits.lo = (bits.lo | (this.buf[this.pos] & 127) << i * 7) >>> 0;
                if (this.buf[this.pos++] < 128)
                    return bits;
            }
            // 4th
            bits.lo = (bits.lo | (this.buf[this.pos++] & 127) << i * 7) >>> 0;
            return bits;
        }
        if (this.len - this.pos > 4) { // fast route (hi)
            for (; i < 5; ++i) {
                // 6th..10th
                bits.hi = (bits.hi | (this.buf[this.pos] & 127) << i * 7 + 3) >>> 0;
                if (this.buf[this.pos++] < 128)
                    return bits;
            }
        } else {
            for (; i < 5; ++i) {
                /* istanbul ignore if */
                if (this.pos >= this.len)
                    throw indexOutOfRange(this);
                // 6th..10th
                bits.hi = (bits.hi | (this.buf[this.pos] & 127) << i * 7 + 3) >>> 0;
                if (this.buf[this.pos++] < 128)
                    return bits;
            }
        }
        /* istanbul ignore next */
        throw Error("invalid varint encoding");
    }

    /* eslint-enable no-invalid-this */

    /**
     * Reads a varint as a signed 64 bit value.
     * @name Reader#int64
     * @function
     * @returns {Long} Value read
     */

    /**
     * Reads a varint as an unsigned 64 bit value.
     * @name Reader#uint64
     * @function
     * @returns {Long} Value read
     */

    /**
     * Reads a zig-zag encoded varint as a signed 64 bit value.
     * @name Reader#sint64
     * @function
     * @returns {Long} Value read
     */

    /**
     * Reads a varint as a boolean.
     * @returns {boolean} Value read
     */
    Reader$1.prototype.bool = function read_bool() {
        return this.uint32() !== 0;
    };

    function readFixed32_end(buf, end) { // note that this uses `end`, not `pos`
        return (buf[end - 4]
              | buf[end - 3] << 8
              | buf[end - 2] << 16
              | buf[end - 1] << 24) >>> 0;
    }

    /**
     * Reads fixed 32 bits as an unsigned 32 bit integer.
     * @returns {number} Value read
     */
    Reader$1.prototype.fixed32 = function read_fixed32() {

        /* istanbul ignore if */
        if (this.pos + 4 > this.len)
            throw indexOutOfRange(this, 4);

        return readFixed32_end(this.buf, this.pos += 4);
    };

    /**
     * Reads fixed 32 bits as a signed 32 bit integer.
     * @returns {number} Value read
     */
    Reader$1.prototype.sfixed32 = function read_sfixed32() {

        /* istanbul ignore if */
        if (this.pos + 4 > this.len)
            throw indexOutOfRange(this, 4);

        return readFixed32_end(this.buf, this.pos += 4) | 0;
    };

    /* eslint-disable no-invalid-this */

    function readFixed64(/* this: Reader */) {

        /* istanbul ignore if */
        if (this.pos + 8 > this.len)
            throw indexOutOfRange(this, 8);

        return new LongBits(readFixed32_end(this.buf, this.pos += 4), readFixed32_end(this.buf, this.pos += 4));
    }

    /* eslint-enable no-invalid-this */

    /**
     * Reads fixed 64 bits.
     * @name Reader#fixed64
     * @function
     * @returns {Long} Value read
     */

    /**
     * Reads zig-zag encoded fixed 64 bits.
     * @name Reader#sfixed64
     * @function
     * @returns {Long} Value read
     */

    /**
     * Reads a float (32 bit) as a number.
     * @function
     * @returns {number} Value read
     */
    Reader$1.prototype.float = function read_float() {

        /* istanbul ignore if */
        if (this.pos + 4 > this.len)
            throw indexOutOfRange(this, 4);

        var value = util$2.float.readFloatLE(this.buf, this.pos);
        this.pos += 4;
        return value;
    };

    /**
     * Reads a double (64 bit float) as a number.
     * @function
     * @returns {number} Value read
     */
    Reader$1.prototype.double = function read_double() {

        /* istanbul ignore if */
        if (this.pos + 8 > this.len)
            throw indexOutOfRange(this, 4);

        var value = util$2.float.readDoubleLE(this.buf, this.pos);
        this.pos += 8;
        return value;
    };

    /**
     * Reads a sequence of bytes preceeded by its length as a varint.
     * @returns {Uint8Array} Value read
     */
    Reader$1.prototype.bytes = function read_bytes() {
        var length = this.uint32(),
            start  = this.pos,
            end    = this.pos + length;

        /* istanbul ignore if */
        if (end > this.len)
            throw indexOutOfRange(this, length);

        this.pos += length;
        if (Array.isArray(this.buf)) // plain array
            return this.buf.slice(start, end);
        return start === end // fix for IE 10/Win8 and others' subarray returning array of size 1
            ? new this.buf.constructor(0)
            : this._slice.call(this.buf, start, end);
    };

    /**
     * Reads a string preceeded by its byte length as a varint.
     * @returns {string} Value read
     */
    Reader$1.prototype.string = function read_string() {
        var bytes = this.bytes();
        return utf8.read(bytes, 0, bytes.length);
    };

    /**
     * Skips the specified number of bytes if specified, otherwise skips a varint.
     * @param {number} [length] Length if known, otherwise a varint is assumed
     * @returns {Reader} `this`
     */
    Reader$1.prototype.skip = function skip(length) {
        if (typeof length === "number") {
            /* istanbul ignore if */
            if (this.pos + length > this.len)
                throw indexOutOfRange(this, length);
            this.pos += length;
        } else {
            do {
                /* istanbul ignore if */
                if (this.pos >= this.len)
                    throw indexOutOfRange(this);
            } while (this.buf[this.pos++] & 128);
        }
        return this;
    };

    /**
     * Skips the next element of the specified wire type.
     * @param {number} wireType Wire type received
     * @returns {Reader} `this`
     */
    Reader$1.prototype.skipType = function(wireType) {
        switch (wireType) {
            case 0:
                this.skip();
                break;
            case 1:
                this.skip(8);
                break;
            case 2:
                this.skip(this.uint32());
                break;
            case 3:
                while ((wireType = this.uint32() & 7) !== 4) {
                    this.skipType(wireType);
                }
                break;
            case 5:
                this.skip(4);
                break;

            /* istanbul ignore next */
            default:
                throw Error("invalid wire type " + wireType + " at offset " + this.pos);
        }
        return this;
    };

    Reader$1._configure = function(BufferReader_) {
        BufferReader$1 = BufferReader_;
        Reader$1.create = create();
        BufferReader$1._configure();

        var fn = util$2.Long ? "toLong" : /* istanbul ignore next */ "toNumber";
        util$2.merge(Reader$1.prototype, {

            int64: function read_int64() {
                return readLongVarint.call(this)[fn](false);
            },

            uint64: function read_uint64() {
                return readLongVarint.call(this)[fn](true);
            },

            sint64: function read_sint64() {
                return readLongVarint.call(this).zzDecode()[fn](false);
            },

            fixed64: function read_fixed64() {
                return readFixed64.call(this)[fn](true);
            },

            sfixed64: function read_sfixed64() {
                return readFixed64.call(this)[fn](false);
            }

        });
    };

    var reader_buffer = BufferReader;

    // extends Reader
    var Reader = reader;
    (BufferReader.prototype = Object.create(Reader.prototype)).constructor = BufferReader;

    var util$1 = requireMinimal();

    /**
     * Constructs a new buffer reader instance.
     * @classdesc Wire format reader using node buffers.
     * @extends Reader
     * @constructor
     * @param {Buffer} buffer Buffer to read from
     */
    function BufferReader(buffer) {
        Reader.call(this, buffer);

        /**
         * Read buffer.
         * @name BufferReader#buf
         * @type {Buffer}
         */
    }

    BufferReader._configure = function () {
        /* istanbul ignore else */
        if (util$1.Buffer)
            BufferReader.prototype._slice = util$1.Buffer.prototype.slice;
    };


    /**
     * @override
     */
    BufferReader.prototype.string = function read_string_buffer() {
        var len = this.uint32(); // modifies pos
        return this.buf.utf8Slice
            ? this.buf.utf8Slice(this.pos, this.pos = Math.min(this.pos + len, this.len))
            : this.buf.toString("utf-8", this.pos, this.pos = Math.min(this.pos + len, this.len));
    };

    /**
     * Reads a sequence of bytes preceeded by its length as a varint.
     * @name BufferReader#bytes
     * @function
     * @returns {Buffer} Value read
     */

    BufferReader._configure();

    var rpc = {};

    var service = Service;

    var util = requireMinimal();

    // Extends EventEmitter
    (Service.prototype = Object.create(util.EventEmitter.prototype)).constructor = Service;

    /**
     * A service method callback as used by {@link rpc.ServiceMethod|ServiceMethod}.
     *
     * Differs from {@link RPCImplCallback} in that it is an actual callback of a service method which may not return `response = null`.
     * @typedef rpc.ServiceMethodCallback
     * @template TRes extends Message<TRes>
     * @type {function}
     * @param {Error|null} error Error, if any
     * @param {TRes} [response] Response message
     * @returns {undefined}
     */

    /**
     * A service method part of a {@link rpc.Service} as created by {@link Service.create}.
     * @typedef rpc.ServiceMethod
     * @template TReq extends Message<TReq>
     * @template TRes extends Message<TRes>
     * @type {function}
     * @param {TReq|Properties<TReq>} request Request message or plain object
     * @param {rpc.ServiceMethodCallback<TRes>} [callback] Node-style callback called with the error, if any, and the response message
     * @returns {Promise<Message<TRes>>} Promise if `callback` has been omitted, otherwise `undefined`
     */

    /**
     * Constructs a new RPC service instance.
     * @classdesc An RPC service as returned by {@link Service#create}.
     * @exports rpc.Service
     * @extends util.EventEmitter
     * @constructor
     * @param {RPCImpl} rpcImpl RPC implementation
     * @param {boolean} [requestDelimited=false] Whether requests are length-delimited
     * @param {boolean} [responseDelimited=false] Whether responses are length-delimited
     */
    function Service(rpcImpl, requestDelimited, responseDelimited) {

        if (typeof rpcImpl !== "function")
            throw TypeError("rpcImpl must be a function");

        util.EventEmitter.call(this);

        /**
         * RPC implementation. Becomes `null` once the service is ended.
         * @type {RPCImpl|null}
         */
        this.rpcImpl = rpcImpl;

        /**
         * Whether requests are length-delimited.
         * @type {boolean}
         */
        this.requestDelimited = Boolean(requestDelimited);

        /**
         * Whether responses are length-delimited.
         * @type {boolean}
         */
        this.responseDelimited = Boolean(responseDelimited);
    }

    /**
     * Calls a service method through {@link rpc.Service#rpcImpl|rpcImpl}.
     * @param {Method|rpc.ServiceMethod<TReq,TRes>} method Reflected or static method
     * @param {Constructor<TReq>} requestCtor Request constructor
     * @param {Constructor<TRes>} responseCtor Response constructor
     * @param {TReq|Properties<TReq>} request Request message or plain object
     * @param {rpc.ServiceMethodCallback<TRes>} callback Service callback
     * @returns {undefined}
     * @template TReq extends Message<TReq>
     * @template TRes extends Message<TRes>
     */
    Service.prototype.rpcCall = function rpcCall(method, requestCtor, responseCtor, request, callback) {

        if (!request)
            throw TypeError("request must be specified");

        var self = this;
        if (!callback)
            return util.asPromise(rpcCall, self, method, requestCtor, responseCtor, request);

        if (!self.rpcImpl) {
            setTimeout(function() { callback(Error("already ended")); }, 0);
            return undefined;
        }

        try {
            return self.rpcImpl(
                method,
                requestCtor[self.requestDelimited ? "encodeDelimited" : "encode"](request).finish(),
                function rpcCallback(err, response) {

                    if (err) {
                        self.emit("error", err, method);
                        return callback(err);
                    }

                    if (response === null) {
                        self.end(/* endedByRPC */ true);
                        return undefined;
                    }

                    if (!(response instanceof responseCtor)) {
                        try {
                            response = responseCtor[self.responseDelimited ? "decodeDelimited" : "decode"](response);
                        } catch (err) {
                            self.emit("error", err, method);
                            return callback(err);
                        }
                    }

                    self.emit("data", response, method);
                    return callback(null, response);
                }
            );
        } catch (err) {
            self.emit("error", err, method);
            setTimeout(function() { callback(err); }, 0);
            return undefined;
        }
    };

    /**
     * Ends this service and emits the `end` event.
     * @param {boolean} [endedByRPC=false] Whether the service has been ended by the RPC implementation.
     * @returns {rpc.Service} `this`
     */
    Service.prototype.end = function end(endedByRPC) {
        if (this.rpcImpl) {
            if (!endedByRPC) // signal end to rpcImpl
                this.rpcImpl(null, null, null);
            this.rpcImpl = null;
            this.emit("end").off();
        }
        return this;
    };

    (function (exports) {

    	/**
    	 * Streaming RPC helpers.
    	 * @namespace
    	 */
    	var rpc = exports;

    	/**
    	 * RPC implementation passed to {@link Service#create} performing a service request on network level, i.e. by utilizing http requests or websockets.
    	 * @typedef RPCImpl
    	 * @type {function}
    	 * @param {Method|rpc.ServiceMethod<Message<{}>,Message<{}>>} method Reflected or static method being called
    	 * @param {Uint8Array} requestData Request data
    	 * @param {RPCImplCallback} callback Callback function
    	 * @returns {undefined}
    	 * @example
    	 * function rpcImpl(method, requestData, callback) {
    	 *     if (protobuf.util.lcFirst(method.name) !== "myMethod") // compatible with static code
    	 *         throw Error("no such method");
    	 *     asynchronouslyObtainAResponse(requestData, function(err, responseData) {
    	 *         callback(err, responseData);
    	 *     });
    	 * }
    	 */

    	/**
    	 * Node-style callback as used by {@link RPCImpl}.
    	 * @typedef RPCImplCallback
    	 * @type {function}
    	 * @param {Error|null} error Error, if any, otherwise `null`
    	 * @param {Uint8Array|null} [response] Response data or `null` to signal end of stream, if there hasn't been an error
    	 * @returns {undefined}
    	 */

    	rpc.Service = service; 
    } (rpc));

    var roots = {};

    (function (exports) {
    	var protobuf = exports;

    	/**
    	 * Build type, one of `"full"`, `"light"` or `"minimal"`.
    	 * @name build
    	 * @type {string}
    	 * @const
    	 */
    	protobuf.build = "minimal";

    	// Serialization
    	protobuf.Writer       = writer;
    	protobuf.BufferWriter = writer_buffer;
    	protobuf.Reader       = reader;
    	protobuf.BufferReader = reader_buffer;

    	// Utility
    	protobuf.util         = requireMinimal();
    	protobuf.rpc          = rpc;
    	protobuf.roots        = roots;
    	protobuf.configure    = configure;

    	/* istanbul ignore next */
    	/**
    	 * Reconfigures the library according to the environment.
    	 * @returns {undefined}
    	 */
    	function configure() {
    	    protobuf.util._configure();
    	    protobuf.Writer._configure(protobuf.BufferWriter);
    	    protobuf.Reader._configure(protobuf.BufferReader);
    	}

    	// Set up buffer utility according to the environment
    	configure(); 
    } (indexMinimal));

    var minimal = indexMinimal;

    var helpers = {};

    var long;
    var hasRequiredLong;

    function requireLong () {
    	if (hasRequiredLong) return long;
    	hasRequiredLong = 1;
    	long = Long;

    	/**
    	 * wasm optimizations, to do native i64 multiplication and divide
    	 */
    	var wasm = null;

    	try {
    	  wasm = new WebAssembly.Instance(new WebAssembly.Module(new Uint8Array([
    	    0, 97, 115, 109, 1, 0, 0, 0, 1, 13, 2, 96, 0, 1, 127, 96, 4, 127, 127, 127, 127, 1, 127, 3, 7, 6, 0, 1, 1, 1, 1, 1, 6, 6, 1, 127, 1, 65, 0, 11, 7, 50, 6, 3, 109, 117, 108, 0, 1, 5, 100, 105, 118, 95, 115, 0, 2, 5, 100, 105, 118, 95, 117, 0, 3, 5, 114, 101, 109, 95, 115, 0, 4, 5, 114, 101, 109, 95, 117, 0, 5, 8, 103, 101, 116, 95, 104, 105, 103, 104, 0, 0, 10, 191, 1, 6, 4, 0, 35, 0, 11, 36, 1, 1, 126, 32, 0, 173, 32, 1, 173, 66, 32, 134, 132, 32, 2, 173, 32, 3, 173, 66, 32, 134, 132, 126, 34, 4, 66, 32, 135, 167, 36, 0, 32, 4, 167, 11, 36, 1, 1, 126, 32, 0, 173, 32, 1, 173, 66, 32, 134, 132, 32, 2, 173, 32, 3, 173, 66, 32, 134, 132, 127, 34, 4, 66, 32, 135, 167, 36, 0, 32, 4, 167, 11, 36, 1, 1, 126, 32, 0, 173, 32, 1, 173, 66, 32, 134, 132, 32, 2, 173, 32, 3, 173, 66, 32, 134, 132, 128, 34, 4, 66, 32, 135, 167, 36, 0, 32, 4, 167, 11, 36, 1, 1, 126, 32, 0, 173, 32, 1, 173, 66, 32, 134, 132, 32, 2, 173, 32, 3, 173, 66, 32, 134, 132, 129, 34, 4, 66, 32, 135, 167, 36, 0, 32, 4, 167, 11, 36, 1, 1, 126, 32, 0, 173, 32, 1, 173, 66, 32, 134, 132, 32, 2, 173, 32, 3, 173, 66, 32, 134, 132, 130, 34, 4, 66, 32, 135, 167, 36, 0, 32, 4, 167, 11
    	  ])), {}).exports;
    	} catch (e) {
    	  // no wasm support :(
    	}

    	/**
    	 * Constructs a 64 bit two's-complement integer, given its low and high 32 bit values as *signed* integers.
    	 *  See the from* functions below for more convenient ways of constructing Longs.
    	 * @exports Long
    	 * @class A Long class for representing a 64 bit two's-complement integer value.
    	 * @param {number} low The low (signed) 32 bits of the long
    	 * @param {number} high The high (signed) 32 bits of the long
    	 * @param {boolean=} unsigned Whether unsigned or not, defaults to signed
    	 * @constructor
    	 */
    	function Long(low, high, unsigned) {

    	    /**
    	     * The low 32 bits as a signed value.
    	     * @type {number}
    	     */
    	    this.low = low | 0;

    	    /**
    	     * The high 32 bits as a signed value.
    	     * @type {number}
    	     */
    	    this.high = high | 0;

    	    /**
    	     * Whether unsigned or not.
    	     * @type {boolean}
    	     */
    	    this.unsigned = !!unsigned;
    	}

    	// The internal representation of a long is the two given signed, 32-bit values.
    	// We use 32-bit pieces because these are the size of integers on which
    	// Javascript performs bit-operations.  For operations like addition and
    	// multiplication, we split each number into 16 bit pieces, which can easily be
    	// multiplied within Javascript's floating-point representation without overflow
    	// or change in sign.
    	//
    	// In the algorithms below, we frequently reduce the negative case to the
    	// positive case by negating the input(s) and then post-processing the result.
    	// Note that we must ALWAYS check specially whether those values are MIN_VALUE
    	// (-2^63) because -MIN_VALUE == MIN_VALUE (since 2^63 cannot be represented as
    	// a positive number, it overflows back into a negative).  Not handling this
    	// case would often result in infinite recursion.
    	//
    	// Common constant values ZERO, ONE, NEG_ONE, etc. are defined below the from*
    	// methods on which they depend.

    	/**
    	 * An indicator used to reliably determine if an object is a Long or not.
    	 * @type {boolean}
    	 * @const
    	 * @private
    	 */
    	Long.prototype.__isLong__;

    	Object.defineProperty(Long.prototype, "__isLong__", { value: true });

    	/**
    	 * @function
    	 * @param {*} obj Object
    	 * @returns {boolean}
    	 * @inner
    	 */
    	function isLong(obj) {
    	    return (obj && obj["__isLong__"]) === true;
    	}

    	/**
    	 * Tests if the specified object is a Long.
    	 * @function
    	 * @param {*} obj Object
    	 * @returns {boolean}
    	 */
    	Long.isLong = isLong;

    	/**
    	 * A cache of the Long representations of small integer values.
    	 * @type {!Object}
    	 * @inner
    	 */
    	var INT_CACHE = {};

    	/**
    	 * A cache of the Long representations of small unsigned integer values.
    	 * @type {!Object}
    	 * @inner
    	 */
    	var UINT_CACHE = {};

    	/**
    	 * @param {number} value
    	 * @param {boolean=} unsigned
    	 * @returns {!Long}
    	 * @inner
    	 */
    	function fromInt(value, unsigned) {
    	    var obj, cachedObj, cache;
    	    if (unsigned) {
    	        value >>>= 0;
    	        if (cache = (0 <= value && value < 256)) {
    	            cachedObj = UINT_CACHE[value];
    	            if (cachedObj)
    	                return cachedObj;
    	        }
    	        obj = fromBits(value, (value | 0) < 0 ? -1 : 0, true);
    	        if (cache)
    	            UINT_CACHE[value] = obj;
    	        return obj;
    	    } else {
    	        value |= 0;
    	        if (cache = (-128 <= value && value < 128)) {
    	            cachedObj = INT_CACHE[value];
    	            if (cachedObj)
    	                return cachedObj;
    	        }
    	        obj = fromBits(value, value < 0 ? -1 : 0, false);
    	        if (cache)
    	            INT_CACHE[value] = obj;
    	        return obj;
    	    }
    	}

    	/**
    	 * Returns a Long representing the given 32 bit integer value.
    	 * @function
    	 * @param {number} value The 32 bit integer in question
    	 * @param {boolean=} unsigned Whether unsigned or not, defaults to signed
    	 * @returns {!Long} The corresponding Long value
    	 */
    	Long.fromInt = fromInt;

    	/**
    	 * @param {number} value
    	 * @param {boolean=} unsigned
    	 * @returns {!Long}
    	 * @inner
    	 */
    	function fromNumber(value, unsigned) {
    	    if (isNaN(value))
    	        return unsigned ? UZERO : ZERO;
    	    if (unsigned) {
    	        if (value < 0)
    	            return UZERO;
    	        if (value >= TWO_PWR_64_DBL)
    	            return MAX_UNSIGNED_VALUE;
    	    } else {
    	        if (value <= -TWO_PWR_63_DBL)
    	            return MIN_VALUE;
    	        if (value + 1 >= TWO_PWR_63_DBL)
    	            return MAX_VALUE;
    	    }
    	    if (value < 0)
    	        return fromNumber(-value, unsigned).neg();
    	    return fromBits((value % TWO_PWR_32_DBL) | 0, (value / TWO_PWR_32_DBL) | 0, unsigned);
    	}

    	/**
    	 * Returns a Long representing the given value, provided that it is a finite number. Otherwise, zero is returned.
    	 * @function
    	 * @param {number} value The number in question
    	 * @param {boolean=} unsigned Whether unsigned or not, defaults to signed
    	 * @returns {!Long} The corresponding Long value
    	 */
    	Long.fromNumber = fromNumber;

    	/**
    	 * @param {number} lowBits
    	 * @param {number} highBits
    	 * @param {boolean=} unsigned
    	 * @returns {!Long}
    	 * @inner
    	 */
    	function fromBits(lowBits, highBits, unsigned) {
    	    return new Long(lowBits, highBits, unsigned);
    	}

    	/**
    	 * Returns a Long representing the 64 bit integer that comes by concatenating the given low and high bits. Each is
    	 *  assumed to use 32 bits.
    	 * @function
    	 * @param {number} lowBits The low 32 bits
    	 * @param {number} highBits The high 32 bits
    	 * @param {boolean=} unsigned Whether unsigned or not, defaults to signed
    	 * @returns {!Long} The corresponding Long value
    	 */
    	Long.fromBits = fromBits;

    	/**
    	 * @function
    	 * @param {number} base
    	 * @param {number} exponent
    	 * @returns {number}
    	 * @inner
    	 */
    	var pow_dbl = Math.pow; // Used 4 times (4*8 to 15+4)

    	/**
    	 * @param {string} str
    	 * @param {(boolean|number)=} unsigned
    	 * @param {number=} radix
    	 * @returns {!Long}
    	 * @inner
    	 */
    	function fromString(str, unsigned, radix) {
    	    if (str.length === 0)
    	        throw Error('empty string');
    	    if (str === "NaN" || str === "Infinity" || str === "+Infinity" || str === "-Infinity")
    	        return ZERO;
    	    if (typeof unsigned === 'number') {
    	        // For goog.math.long compatibility
    	        radix = unsigned,
    	        unsigned = false;
    	    } else {
    	        unsigned = !! unsigned;
    	    }
    	    radix = radix || 10;
    	    if (radix < 2 || 36 < radix)
    	        throw RangeError('radix');

    	    var p;
    	    if ((p = str.indexOf('-')) > 0)
    	        throw Error('interior hyphen');
    	    else if (p === 0) {
    	        return fromString(str.substring(1), unsigned, radix).neg();
    	    }

    	    // Do several (8) digits each time through the loop, so as to
    	    // minimize the calls to the very expensive emulated div.
    	    var radixToPower = fromNumber(pow_dbl(radix, 8));

    	    var result = ZERO;
    	    for (var i = 0; i < str.length; i += 8) {
    	        var size = Math.min(8, str.length - i),
    	            value = parseInt(str.substring(i, i + size), radix);
    	        if (size < 8) {
    	            var power = fromNumber(pow_dbl(radix, size));
    	            result = result.mul(power).add(fromNumber(value));
    	        } else {
    	            result = result.mul(radixToPower);
    	            result = result.add(fromNumber(value));
    	        }
    	    }
    	    result.unsigned = unsigned;
    	    return result;
    	}

    	/**
    	 * Returns a Long representation of the given string, written using the specified radix.
    	 * @function
    	 * @param {string} str The textual representation of the Long
    	 * @param {(boolean|number)=} unsigned Whether unsigned or not, defaults to signed
    	 * @param {number=} radix The radix in which the text is written (2-36), defaults to 10
    	 * @returns {!Long} The corresponding Long value
    	 */
    	Long.fromString = fromString;

    	/**
    	 * @function
    	 * @param {!Long|number|string|!{low: number, high: number, unsigned: boolean}} val
    	 * @param {boolean=} unsigned
    	 * @returns {!Long}
    	 * @inner
    	 */
    	function fromValue(val, unsigned) {
    	    if (typeof val === 'number')
    	        return fromNumber(val, unsigned);
    	    if (typeof val === 'string')
    	        return fromString(val, unsigned);
    	    // Throws for non-objects, converts non-instanceof Long:
    	    return fromBits(val.low, val.high, typeof unsigned === 'boolean' ? unsigned : val.unsigned);
    	}

    	/**
    	 * Converts the specified value to a Long using the appropriate from* function for its type.
    	 * @function
    	 * @param {!Long|number|string|!{low: number, high: number, unsigned: boolean}} val Value
    	 * @param {boolean=} unsigned Whether unsigned or not, defaults to signed
    	 * @returns {!Long}
    	 */
    	Long.fromValue = fromValue;

    	// NOTE: the compiler should inline these constant values below and then remove these variables, so there should be
    	// no runtime penalty for these.

    	/**
    	 * @type {number}
    	 * @const
    	 * @inner
    	 */
    	var TWO_PWR_16_DBL = 1 << 16;

    	/**
    	 * @type {number}
    	 * @const
    	 * @inner
    	 */
    	var TWO_PWR_24_DBL = 1 << 24;

    	/**
    	 * @type {number}
    	 * @const
    	 * @inner
    	 */
    	var TWO_PWR_32_DBL = TWO_PWR_16_DBL * TWO_PWR_16_DBL;

    	/**
    	 * @type {number}
    	 * @const
    	 * @inner
    	 */
    	var TWO_PWR_64_DBL = TWO_PWR_32_DBL * TWO_PWR_32_DBL;

    	/**
    	 * @type {number}
    	 * @const
    	 * @inner
    	 */
    	var TWO_PWR_63_DBL = TWO_PWR_64_DBL / 2;

    	/**
    	 * @type {!Long}
    	 * @const
    	 * @inner
    	 */
    	var TWO_PWR_24 = fromInt(TWO_PWR_24_DBL);

    	/**
    	 * @type {!Long}
    	 * @inner
    	 */
    	var ZERO = fromInt(0);

    	/**
    	 * Signed zero.
    	 * @type {!Long}
    	 */
    	Long.ZERO = ZERO;

    	/**
    	 * @type {!Long}
    	 * @inner
    	 */
    	var UZERO = fromInt(0, true);

    	/**
    	 * Unsigned zero.
    	 * @type {!Long}
    	 */
    	Long.UZERO = UZERO;

    	/**
    	 * @type {!Long}
    	 * @inner
    	 */
    	var ONE = fromInt(1);

    	/**
    	 * Signed one.
    	 * @type {!Long}
    	 */
    	Long.ONE = ONE;

    	/**
    	 * @type {!Long}
    	 * @inner
    	 */
    	var UONE = fromInt(1, true);

    	/**
    	 * Unsigned one.
    	 * @type {!Long}
    	 */
    	Long.UONE = UONE;

    	/**
    	 * @type {!Long}
    	 * @inner
    	 */
    	var NEG_ONE = fromInt(-1);

    	/**
    	 * Signed negative one.
    	 * @type {!Long}
    	 */
    	Long.NEG_ONE = NEG_ONE;

    	/**
    	 * @type {!Long}
    	 * @inner
    	 */
    	var MAX_VALUE = fromBits(0xFFFFFFFF|0, 0x7FFFFFFF|0, false);

    	/**
    	 * Maximum signed value.
    	 * @type {!Long}
    	 */
    	Long.MAX_VALUE = MAX_VALUE;

    	/**
    	 * @type {!Long}
    	 * @inner
    	 */
    	var MAX_UNSIGNED_VALUE = fromBits(0xFFFFFFFF|0, 0xFFFFFFFF|0, true);

    	/**
    	 * Maximum unsigned value.
    	 * @type {!Long}
    	 */
    	Long.MAX_UNSIGNED_VALUE = MAX_UNSIGNED_VALUE;

    	/**
    	 * @type {!Long}
    	 * @inner
    	 */
    	var MIN_VALUE = fromBits(0, 0x80000000|0, false);

    	/**
    	 * Minimum signed value.
    	 * @type {!Long}
    	 */
    	Long.MIN_VALUE = MIN_VALUE;

    	/**
    	 * @alias Long.prototype
    	 * @inner
    	 */
    	var LongPrototype = Long.prototype;

    	/**
    	 * Converts the Long to a 32 bit integer, assuming it is a 32 bit integer.
    	 * @returns {number}
    	 */
    	LongPrototype.toInt = function toInt() {
    	    return this.unsigned ? this.low >>> 0 : this.low;
    	};

    	/**
    	 * Converts the Long to a the nearest floating-point representation of this value (double, 53 bit mantissa).
    	 * @returns {number}
    	 */
    	LongPrototype.toNumber = function toNumber() {
    	    if (this.unsigned)
    	        return ((this.high >>> 0) * TWO_PWR_32_DBL) + (this.low >>> 0);
    	    return this.high * TWO_PWR_32_DBL + (this.low >>> 0);
    	};

    	/**
    	 * Converts the Long to a string written in the specified radix.
    	 * @param {number=} radix Radix (2-36), defaults to 10
    	 * @returns {string}
    	 * @override
    	 * @throws {RangeError} If `radix` is out of range
    	 */
    	LongPrototype.toString = function toString(radix) {
    	    radix = radix || 10;
    	    if (radix < 2 || 36 < radix)
    	        throw RangeError('radix');
    	    if (this.isZero())
    	        return '0';
    	    if (this.isNegative()) { // Unsigned Longs are never negative
    	        if (this.eq(MIN_VALUE)) {
    	            // We need to change the Long value before it can be negated, so we remove
    	            // the bottom-most digit in this base and then recurse to do the rest.
    	            var radixLong = fromNumber(radix),
    	                div = this.div(radixLong),
    	                rem1 = div.mul(radixLong).sub(this);
    	            return div.toString(radix) + rem1.toInt().toString(radix);
    	        } else
    	            return '-' + this.neg().toString(radix);
    	    }

    	    // Do several (6) digits each time through the loop, so as to
    	    // minimize the calls to the very expensive emulated div.
    	    var radixToPower = fromNumber(pow_dbl(radix, 6), this.unsigned),
    	        rem = this;
    	    var result = '';
    	    while (true) {
    	        var remDiv = rem.div(radixToPower),
    	            intval = rem.sub(remDiv.mul(radixToPower)).toInt() >>> 0,
    	            digits = intval.toString(radix);
    	        rem = remDiv;
    	        if (rem.isZero())
    	            return digits + result;
    	        else {
    	            while (digits.length < 6)
    	                digits = '0' + digits;
    	            result = '' + digits + result;
    	        }
    	    }
    	};

    	/**
    	 * Gets the high 32 bits as a signed integer.
    	 * @returns {number} Signed high bits
    	 */
    	LongPrototype.getHighBits = function getHighBits() {
    	    return this.high;
    	};

    	/**
    	 * Gets the high 32 bits as an unsigned integer.
    	 * @returns {number} Unsigned high bits
    	 */
    	LongPrototype.getHighBitsUnsigned = function getHighBitsUnsigned() {
    	    return this.high >>> 0;
    	};

    	/**
    	 * Gets the low 32 bits as a signed integer.
    	 * @returns {number} Signed low bits
    	 */
    	LongPrototype.getLowBits = function getLowBits() {
    	    return this.low;
    	};

    	/**
    	 * Gets the low 32 bits as an unsigned integer.
    	 * @returns {number} Unsigned low bits
    	 */
    	LongPrototype.getLowBitsUnsigned = function getLowBitsUnsigned() {
    	    return this.low >>> 0;
    	};

    	/**
    	 * Gets the number of bits needed to represent the absolute value of this Long.
    	 * @returns {number}
    	 */
    	LongPrototype.getNumBitsAbs = function getNumBitsAbs() {
    	    if (this.isNegative()) // Unsigned Longs are never negative
    	        return this.eq(MIN_VALUE) ? 64 : this.neg().getNumBitsAbs();
    	    var val = this.high != 0 ? this.high : this.low;
    	    for (var bit = 31; bit > 0; bit--)
    	        if ((val & (1 << bit)) != 0)
    	            break;
    	    return this.high != 0 ? bit + 33 : bit + 1;
    	};

    	/**
    	 * Tests if this Long's value equals zero.
    	 * @returns {boolean}
    	 */
    	LongPrototype.isZero = function isZero() {
    	    return this.high === 0 && this.low === 0;
    	};

    	/**
    	 * Tests if this Long's value equals zero. This is an alias of {@link Long#isZero}.
    	 * @returns {boolean}
    	 */
    	LongPrototype.eqz = LongPrototype.isZero;

    	/**
    	 * Tests if this Long's value is negative.
    	 * @returns {boolean}
    	 */
    	LongPrototype.isNegative = function isNegative() {
    	    return !this.unsigned && this.high < 0;
    	};

    	/**
    	 * Tests if this Long's value is positive.
    	 * @returns {boolean}
    	 */
    	LongPrototype.isPositive = function isPositive() {
    	    return this.unsigned || this.high >= 0;
    	};

    	/**
    	 * Tests if this Long's value is odd.
    	 * @returns {boolean}
    	 */
    	LongPrototype.isOdd = function isOdd() {
    	    return (this.low & 1) === 1;
    	};

    	/**
    	 * Tests if this Long's value is even.
    	 * @returns {boolean}
    	 */
    	LongPrototype.isEven = function isEven() {
    	    return (this.low & 1) === 0;
    	};

    	/**
    	 * Tests if this Long's value equals the specified's.
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.equals = function equals(other) {
    	    if (!isLong(other))
    	        other = fromValue(other);
    	    if (this.unsigned !== other.unsigned && (this.high >>> 31) === 1 && (other.high >>> 31) === 1)
    	        return false;
    	    return this.high === other.high && this.low === other.low;
    	};

    	/**
    	 * Tests if this Long's value equals the specified's. This is an alias of {@link Long#equals}.
    	 * @function
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.eq = LongPrototype.equals;

    	/**
    	 * Tests if this Long's value differs from the specified's.
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.notEquals = function notEquals(other) {
    	    return !this.eq(/* validates */ other);
    	};

    	/**
    	 * Tests if this Long's value differs from the specified's. This is an alias of {@link Long#notEquals}.
    	 * @function
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.neq = LongPrototype.notEquals;

    	/**
    	 * Tests if this Long's value differs from the specified's. This is an alias of {@link Long#notEquals}.
    	 * @function
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.ne = LongPrototype.notEquals;

    	/**
    	 * Tests if this Long's value is less than the specified's.
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.lessThan = function lessThan(other) {
    	    return this.comp(/* validates */ other) < 0;
    	};

    	/**
    	 * Tests if this Long's value is less than the specified's. This is an alias of {@link Long#lessThan}.
    	 * @function
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.lt = LongPrototype.lessThan;

    	/**
    	 * Tests if this Long's value is less than or equal the specified's.
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.lessThanOrEqual = function lessThanOrEqual(other) {
    	    return this.comp(/* validates */ other) <= 0;
    	};

    	/**
    	 * Tests if this Long's value is less than or equal the specified's. This is an alias of {@link Long#lessThanOrEqual}.
    	 * @function
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.lte = LongPrototype.lessThanOrEqual;

    	/**
    	 * Tests if this Long's value is less than or equal the specified's. This is an alias of {@link Long#lessThanOrEqual}.
    	 * @function
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.le = LongPrototype.lessThanOrEqual;

    	/**
    	 * Tests if this Long's value is greater than the specified's.
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.greaterThan = function greaterThan(other) {
    	    return this.comp(/* validates */ other) > 0;
    	};

    	/**
    	 * Tests if this Long's value is greater than the specified's. This is an alias of {@link Long#greaterThan}.
    	 * @function
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.gt = LongPrototype.greaterThan;

    	/**
    	 * Tests if this Long's value is greater than or equal the specified's.
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.greaterThanOrEqual = function greaterThanOrEqual(other) {
    	    return this.comp(/* validates */ other) >= 0;
    	};

    	/**
    	 * Tests if this Long's value is greater than or equal the specified's. This is an alias of {@link Long#greaterThanOrEqual}.
    	 * @function
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.gte = LongPrototype.greaterThanOrEqual;

    	/**
    	 * Tests if this Long's value is greater than or equal the specified's. This is an alias of {@link Long#greaterThanOrEqual}.
    	 * @function
    	 * @param {!Long|number|string} other Other value
    	 * @returns {boolean}
    	 */
    	LongPrototype.ge = LongPrototype.greaterThanOrEqual;

    	/**
    	 * Compares this Long's value with the specified's.
    	 * @param {!Long|number|string} other Other value
    	 * @returns {number} 0 if they are the same, 1 if the this is greater and -1
    	 *  if the given one is greater
    	 */
    	LongPrototype.compare = function compare(other) {
    	    if (!isLong(other))
    	        other = fromValue(other);
    	    if (this.eq(other))
    	        return 0;
    	    var thisNeg = this.isNegative(),
    	        otherNeg = other.isNegative();
    	    if (thisNeg && !otherNeg)
    	        return -1;
    	    if (!thisNeg && otherNeg)
    	        return 1;
    	    // At this point the sign bits are the same
    	    if (!this.unsigned)
    	        return this.sub(other).isNegative() ? -1 : 1;
    	    // Both are positive if at least one is unsigned
    	    return (other.high >>> 0) > (this.high >>> 0) || (other.high === this.high && (other.low >>> 0) > (this.low >>> 0)) ? -1 : 1;
    	};

    	/**
    	 * Compares this Long's value with the specified's. This is an alias of {@link Long#compare}.
    	 * @function
    	 * @param {!Long|number|string} other Other value
    	 * @returns {number} 0 if they are the same, 1 if the this is greater and -1
    	 *  if the given one is greater
    	 */
    	LongPrototype.comp = LongPrototype.compare;

    	/**
    	 * Negates this Long's value.
    	 * @returns {!Long} Negated Long
    	 */
    	LongPrototype.negate = function negate() {
    	    if (!this.unsigned && this.eq(MIN_VALUE))
    	        return MIN_VALUE;
    	    return this.not().add(ONE);
    	};

    	/**
    	 * Negates this Long's value. This is an alias of {@link Long#negate}.
    	 * @function
    	 * @returns {!Long} Negated Long
    	 */
    	LongPrototype.neg = LongPrototype.negate;

    	/**
    	 * Returns the sum of this and the specified Long.
    	 * @param {!Long|number|string} addend Addend
    	 * @returns {!Long} Sum
    	 */
    	LongPrototype.add = function add(addend) {
    	    if (!isLong(addend))
    	        addend = fromValue(addend);

    	    // Divide each number into 4 chunks of 16 bits, and then sum the chunks.

    	    var a48 = this.high >>> 16;
    	    var a32 = this.high & 0xFFFF;
    	    var a16 = this.low >>> 16;
    	    var a00 = this.low & 0xFFFF;

    	    var b48 = addend.high >>> 16;
    	    var b32 = addend.high & 0xFFFF;
    	    var b16 = addend.low >>> 16;
    	    var b00 = addend.low & 0xFFFF;

    	    var c48 = 0, c32 = 0, c16 = 0, c00 = 0;
    	    c00 += a00 + b00;
    	    c16 += c00 >>> 16;
    	    c00 &= 0xFFFF;
    	    c16 += a16 + b16;
    	    c32 += c16 >>> 16;
    	    c16 &= 0xFFFF;
    	    c32 += a32 + b32;
    	    c48 += c32 >>> 16;
    	    c32 &= 0xFFFF;
    	    c48 += a48 + b48;
    	    c48 &= 0xFFFF;
    	    return fromBits((c16 << 16) | c00, (c48 << 16) | c32, this.unsigned);
    	};

    	/**
    	 * Returns the difference of this and the specified Long.
    	 * @param {!Long|number|string} subtrahend Subtrahend
    	 * @returns {!Long} Difference
    	 */
    	LongPrototype.subtract = function subtract(subtrahend) {
    	    if (!isLong(subtrahend))
    	        subtrahend = fromValue(subtrahend);
    	    return this.add(subtrahend.neg());
    	};

    	/**
    	 * Returns the difference of this and the specified Long. This is an alias of {@link Long#subtract}.
    	 * @function
    	 * @param {!Long|number|string} subtrahend Subtrahend
    	 * @returns {!Long} Difference
    	 */
    	LongPrototype.sub = LongPrototype.subtract;

    	/**
    	 * Returns the product of this and the specified Long.
    	 * @param {!Long|number|string} multiplier Multiplier
    	 * @returns {!Long} Product
    	 */
    	LongPrototype.multiply = function multiply(multiplier) {
    	    if (this.isZero())
    	        return ZERO;
    	    if (!isLong(multiplier))
    	        multiplier = fromValue(multiplier);

    	    // use wasm support if present
    	    if (wasm) {
    	        var low = wasm.mul(this.low,
    	                           this.high,
    	                           multiplier.low,
    	                           multiplier.high);
    	        return fromBits(low, wasm.get_high(), this.unsigned);
    	    }

    	    if (multiplier.isZero())
    	        return ZERO;
    	    if (this.eq(MIN_VALUE))
    	        return multiplier.isOdd() ? MIN_VALUE : ZERO;
    	    if (multiplier.eq(MIN_VALUE))
    	        return this.isOdd() ? MIN_VALUE : ZERO;

    	    if (this.isNegative()) {
    	        if (multiplier.isNegative())
    	            return this.neg().mul(multiplier.neg());
    	        else
    	            return this.neg().mul(multiplier).neg();
    	    } else if (multiplier.isNegative())
    	        return this.mul(multiplier.neg()).neg();

    	    // If both longs are small, use float multiplication
    	    if (this.lt(TWO_PWR_24) && multiplier.lt(TWO_PWR_24))
    	        return fromNumber(this.toNumber() * multiplier.toNumber(), this.unsigned);

    	    // Divide each long into 4 chunks of 16 bits, and then add up 4x4 products.
    	    // We can skip products that would overflow.

    	    var a48 = this.high >>> 16;
    	    var a32 = this.high & 0xFFFF;
    	    var a16 = this.low >>> 16;
    	    var a00 = this.low & 0xFFFF;

    	    var b48 = multiplier.high >>> 16;
    	    var b32 = multiplier.high & 0xFFFF;
    	    var b16 = multiplier.low >>> 16;
    	    var b00 = multiplier.low & 0xFFFF;

    	    var c48 = 0, c32 = 0, c16 = 0, c00 = 0;
    	    c00 += a00 * b00;
    	    c16 += c00 >>> 16;
    	    c00 &= 0xFFFF;
    	    c16 += a16 * b00;
    	    c32 += c16 >>> 16;
    	    c16 &= 0xFFFF;
    	    c16 += a00 * b16;
    	    c32 += c16 >>> 16;
    	    c16 &= 0xFFFF;
    	    c32 += a32 * b00;
    	    c48 += c32 >>> 16;
    	    c32 &= 0xFFFF;
    	    c32 += a16 * b16;
    	    c48 += c32 >>> 16;
    	    c32 &= 0xFFFF;
    	    c32 += a00 * b32;
    	    c48 += c32 >>> 16;
    	    c32 &= 0xFFFF;
    	    c48 += a48 * b00 + a32 * b16 + a16 * b32 + a00 * b48;
    	    c48 &= 0xFFFF;
    	    return fromBits((c16 << 16) | c00, (c48 << 16) | c32, this.unsigned);
    	};

    	/**
    	 * Returns the product of this and the specified Long. This is an alias of {@link Long#multiply}.
    	 * @function
    	 * @param {!Long|number|string} multiplier Multiplier
    	 * @returns {!Long} Product
    	 */
    	LongPrototype.mul = LongPrototype.multiply;

    	/**
    	 * Returns this Long divided by the specified. The result is signed if this Long is signed or
    	 *  unsigned if this Long is unsigned.
    	 * @param {!Long|number|string} divisor Divisor
    	 * @returns {!Long} Quotient
    	 */
    	LongPrototype.divide = function divide(divisor) {
    	    if (!isLong(divisor))
    	        divisor = fromValue(divisor);
    	    if (divisor.isZero())
    	        throw Error('division by zero');

    	    // use wasm support if present
    	    if (wasm) {
    	        // guard against signed division overflow: the largest
    	        // negative number / -1 would be 1 larger than the largest
    	        // positive number, due to two's complement.
    	        if (!this.unsigned &&
    	            this.high === -0x80000000 &&
    	            divisor.low === -1 && divisor.high === -1) {
    	            // be consistent with non-wasm code path
    	            return this;
    	        }
    	        var low = (this.unsigned ? wasm.div_u : wasm.div_s)(
    	            this.low,
    	            this.high,
    	            divisor.low,
    	            divisor.high
    	        );
    	        return fromBits(low, wasm.get_high(), this.unsigned);
    	    }

    	    if (this.isZero())
    	        return this.unsigned ? UZERO : ZERO;
    	    var approx, rem, res;
    	    if (!this.unsigned) {
    	        // This section is only relevant for signed longs and is derived from the
    	        // closure library as a whole.
    	        if (this.eq(MIN_VALUE)) {
    	            if (divisor.eq(ONE) || divisor.eq(NEG_ONE))
    	                return MIN_VALUE;  // recall that -MIN_VALUE == MIN_VALUE
    	            else if (divisor.eq(MIN_VALUE))
    	                return ONE;
    	            else {
    	                // At this point, we have |other| >= 2, so |this/other| < |MIN_VALUE|.
    	                var halfThis = this.shr(1);
    	                approx = halfThis.div(divisor).shl(1);
    	                if (approx.eq(ZERO)) {
    	                    return divisor.isNegative() ? ONE : NEG_ONE;
    	                } else {
    	                    rem = this.sub(divisor.mul(approx));
    	                    res = approx.add(rem.div(divisor));
    	                    return res;
    	                }
    	            }
    	        } else if (divisor.eq(MIN_VALUE))
    	            return this.unsigned ? UZERO : ZERO;
    	        if (this.isNegative()) {
    	            if (divisor.isNegative())
    	                return this.neg().div(divisor.neg());
    	            return this.neg().div(divisor).neg();
    	        } else if (divisor.isNegative())
    	            return this.div(divisor.neg()).neg();
    	        res = ZERO;
    	    } else {
    	        // The algorithm below has not been made for unsigned longs. It's therefore
    	        // required to take special care of the MSB prior to running it.
    	        if (!divisor.unsigned)
    	            divisor = divisor.toUnsigned();
    	        if (divisor.gt(this))
    	            return UZERO;
    	        if (divisor.gt(this.shru(1))) // 15 >>> 1 = 7 ; with divisor = 8 ; true
    	            return UONE;
    	        res = UZERO;
    	    }

    	    // Repeat the following until the remainder is less than other:  find a
    	    // floating-point that approximates remainder / other *from below*, add this
    	    // into the result, and subtract it from the remainder.  It is critical that
    	    // the approximate value is less than or equal to the real value so that the
    	    // remainder never becomes negative.
    	    rem = this;
    	    while (rem.gte(divisor)) {
    	        // Approximate the result of division. This may be a little greater or
    	        // smaller than the actual value.
    	        approx = Math.max(1, Math.floor(rem.toNumber() / divisor.toNumber()));

    	        // We will tweak the approximate result by changing it in the 48-th digit or
    	        // the smallest non-fractional digit, whichever is larger.
    	        var log2 = Math.ceil(Math.log(approx) / Math.LN2),
    	            delta = (log2 <= 48) ? 1 : pow_dbl(2, log2 - 48),

    	        // Decrease the approximation until it is smaller than the remainder.  Note
    	        // that if it is too large, the product overflows and is negative.
    	            approxRes = fromNumber(approx),
    	            approxRem = approxRes.mul(divisor);
    	        while (approxRem.isNegative() || approxRem.gt(rem)) {
    	            approx -= delta;
    	            approxRes = fromNumber(approx, this.unsigned);
    	            approxRem = approxRes.mul(divisor);
    	        }

    	        // We know the answer can't be zero... and actually, zero would cause
    	        // infinite recursion since we would make no progress.
    	        if (approxRes.isZero())
    	            approxRes = ONE;

    	        res = res.add(approxRes);
    	        rem = rem.sub(approxRem);
    	    }
    	    return res;
    	};

    	/**
    	 * Returns this Long divided by the specified. This is an alias of {@link Long#divide}.
    	 * @function
    	 * @param {!Long|number|string} divisor Divisor
    	 * @returns {!Long} Quotient
    	 */
    	LongPrototype.div = LongPrototype.divide;

    	/**
    	 * Returns this Long modulo the specified.
    	 * @param {!Long|number|string} divisor Divisor
    	 * @returns {!Long} Remainder
    	 */
    	LongPrototype.modulo = function modulo(divisor) {
    	    if (!isLong(divisor))
    	        divisor = fromValue(divisor);

    	    // use wasm support if present
    	    if (wasm) {
    	        var low = (this.unsigned ? wasm.rem_u : wasm.rem_s)(
    	            this.low,
    	            this.high,
    	            divisor.low,
    	            divisor.high
    	        );
    	        return fromBits(low, wasm.get_high(), this.unsigned);
    	    }

    	    return this.sub(this.div(divisor).mul(divisor));
    	};

    	/**
    	 * Returns this Long modulo the specified. This is an alias of {@link Long#modulo}.
    	 * @function
    	 * @param {!Long|number|string} divisor Divisor
    	 * @returns {!Long} Remainder
    	 */
    	LongPrototype.mod = LongPrototype.modulo;

    	/**
    	 * Returns this Long modulo the specified. This is an alias of {@link Long#modulo}.
    	 * @function
    	 * @param {!Long|number|string} divisor Divisor
    	 * @returns {!Long} Remainder
    	 */
    	LongPrototype.rem = LongPrototype.modulo;

    	/**
    	 * Returns the bitwise NOT of this Long.
    	 * @returns {!Long}
    	 */
    	LongPrototype.not = function not() {
    	    return fromBits(~this.low, ~this.high, this.unsigned);
    	};

    	/**
    	 * Returns the bitwise AND of this Long and the specified.
    	 * @param {!Long|number|string} other Other Long
    	 * @returns {!Long}
    	 */
    	LongPrototype.and = function and(other) {
    	    if (!isLong(other))
    	        other = fromValue(other);
    	    return fromBits(this.low & other.low, this.high & other.high, this.unsigned);
    	};

    	/**
    	 * Returns the bitwise OR of this Long and the specified.
    	 * @param {!Long|number|string} other Other Long
    	 * @returns {!Long}
    	 */
    	LongPrototype.or = function or(other) {
    	    if (!isLong(other))
    	        other = fromValue(other);
    	    return fromBits(this.low | other.low, this.high | other.high, this.unsigned);
    	};

    	/**
    	 * Returns the bitwise XOR of this Long and the given one.
    	 * @param {!Long|number|string} other Other Long
    	 * @returns {!Long}
    	 */
    	LongPrototype.xor = function xor(other) {
    	    if (!isLong(other))
    	        other = fromValue(other);
    	    return fromBits(this.low ^ other.low, this.high ^ other.high, this.unsigned);
    	};

    	/**
    	 * Returns this Long with bits shifted to the left by the given amount.
    	 * @param {number|!Long} numBits Number of bits
    	 * @returns {!Long} Shifted Long
    	 */
    	LongPrototype.shiftLeft = function shiftLeft(numBits) {
    	    if (isLong(numBits))
    	        numBits = numBits.toInt();
    	    if ((numBits &= 63) === 0)
    	        return this;
    	    else if (numBits < 32)
    	        return fromBits(this.low << numBits, (this.high << numBits) | (this.low >>> (32 - numBits)), this.unsigned);
    	    else
    	        return fromBits(0, this.low << (numBits - 32), this.unsigned);
    	};

    	/**
    	 * Returns this Long with bits shifted to the left by the given amount. This is an alias of {@link Long#shiftLeft}.
    	 * @function
    	 * @param {number|!Long} numBits Number of bits
    	 * @returns {!Long} Shifted Long
    	 */
    	LongPrototype.shl = LongPrototype.shiftLeft;

    	/**
    	 * Returns this Long with bits arithmetically shifted to the right by the given amount.
    	 * @param {number|!Long} numBits Number of bits
    	 * @returns {!Long} Shifted Long
    	 */
    	LongPrototype.shiftRight = function shiftRight(numBits) {
    	    if (isLong(numBits))
    	        numBits = numBits.toInt();
    	    if ((numBits &= 63) === 0)
    	        return this;
    	    else if (numBits < 32)
    	        return fromBits((this.low >>> numBits) | (this.high << (32 - numBits)), this.high >> numBits, this.unsigned);
    	    else
    	        return fromBits(this.high >> (numBits - 32), this.high >= 0 ? 0 : -1, this.unsigned);
    	};

    	/**
    	 * Returns this Long with bits arithmetically shifted to the right by the given amount. This is an alias of {@link Long#shiftRight}.
    	 * @function
    	 * @param {number|!Long} numBits Number of bits
    	 * @returns {!Long} Shifted Long
    	 */
    	LongPrototype.shr = LongPrototype.shiftRight;

    	/**
    	 * Returns this Long with bits logically shifted to the right by the given amount.
    	 * @param {number|!Long} numBits Number of bits
    	 * @returns {!Long} Shifted Long
    	 */
    	LongPrototype.shiftRightUnsigned = function shiftRightUnsigned(numBits) {
    	    if (isLong(numBits))
    	        numBits = numBits.toInt();
    	    numBits &= 63;
    	    if (numBits === 0)
    	        return this;
    	    else {
    	        var high = this.high;
    	        if (numBits < 32) {
    	            var low = this.low;
    	            return fromBits((low >>> numBits) | (high << (32 - numBits)), high >>> numBits, this.unsigned);
    	        } else if (numBits === 32)
    	            return fromBits(high, 0, this.unsigned);
    	        else
    	            return fromBits(high >>> (numBits - 32), 0, this.unsigned);
    	    }
    	};

    	/**
    	 * Returns this Long with bits logically shifted to the right by the given amount. This is an alias of {@link Long#shiftRightUnsigned}.
    	 * @function
    	 * @param {number|!Long} numBits Number of bits
    	 * @returns {!Long} Shifted Long
    	 */
    	LongPrototype.shru = LongPrototype.shiftRightUnsigned;

    	/**
    	 * Returns this Long with bits logically shifted to the right by the given amount. This is an alias of {@link Long#shiftRightUnsigned}.
    	 * @function
    	 * @param {number|!Long} numBits Number of bits
    	 * @returns {!Long} Shifted Long
    	 */
    	LongPrototype.shr_u = LongPrototype.shiftRightUnsigned;

    	/**
    	 * Converts this Long to signed.
    	 * @returns {!Long} Signed long
    	 */
    	LongPrototype.toSigned = function toSigned() {
    	    if (!this.unsigned)
    	        return this;
    	    return fromBits(this.low, this.high, false);
    	};

    	/**
    	 * Converts this Long to unsigned.
    	 * @returns {!Long} Unsigned long
    	 */
    	LongPrototype.toUnsigned = function toUnsigned() {
    	    if (this.unsigned)
    	        return this;
    	    return fromBits(this.low, this.high, true);
    	};

    	/**
    	 * Converts this Long to its byte representation.
    	 * @param {boolean=} le Whether little or big endian, defaults to big endian
    	 * @returns {!Array.<number>} Byte representation
    	 */
    	LongPrototype.toBytes = function toBytes(le) {
    	    return le ? this.toBytesLE() : this.toBytesBE();
    	};

    	/**
    	 * Converts this Long to its little endian byte representation.
    	 * @returns {!Array.<number>} Little endian byte representation
    	 */
    	LongPrototype.toBytesLE = function toBytesLE() {
    	    var hi = this.high,
    	        lo = this.low;
    	    return [
    	        lo        & 0xff,
    	        lo >>>  8 & 0xff,
    	        lo >>> 16 & 0xff,
    	        lo >>> 24       ,
    	        hi        & 0xff,
    	        hi >>>  8 & 0xff,
    	        hi >>> 16 & 0xff,
    	        hi >>> 24
    	    ];
    	};

    	/**
    	 * Converts this Long to its big endian byte representation.
    	 * @returns {!Array.<number>} Big endian byte representation
    	 */
    	LongPrototype.toBytesBE = function toBytesBE() {
    	    var hi = this.high,
    	        lo = this.low;
    	    return [
    	        hi >>> 24       ,
    	        hi >>> 16 & 0xff,
    	        hi >>>  8 & 0xff,
    	        hi        & 0xff,
    	        lo >>> 24       ,
    	        lo >>> 16 & 0xff,
    	        lo >>>  8 & 0xff,
    	        lo        & 0xff
    	    ];
    	};

    	/**
    	 * Creates a Long from its byte representation.
    	 * @param {!Array.<number>} bytes Byte representation
    	 * @param {boolean=} unsigned Whether unsigned or not, defaults to signed
    	 * @param {boolean=} le Whether little or big endian, defaults to big endian
    	 * @returns {Long} The corresponding Long value
    	 */
    	Long.fromBytes = function fromBytes(bytes, unsigned, le) {
    	    return le ? Long.fromBytesLE(bytes, unsigned) : Long.fromBytesBE(bytes, unsigned);
    	};

    	/**
    	 * Creates a Long from its little endian byte representation.
    	 * @param {!Array.<number>} bytes Little endian byte representation
    	 * @param {boolean=} unsigned Whether unsigned or not, defaults to signed
    	 * @returns {Long} The corresponding Long value
    	 */
    	Long.fromBytesLE = function fromBytesLE(bytes, unsigned) {
    	    return new Long(
    	        bytes[0]       |
    	        bytes[1] <<  8 |
    	        bytes[2] << 16 |
    	        bytes[3] << 24,
    	        bytes[4]       |
    	        bytes[5] <<  8 |
    	        bytes[6] << 16 |
    	        bytes[7] << 24,
    	        unsigned
    	    );
    	};

    	/**
    	 * Creates a Long from its big endian byte representation.
    	 * @param {!Array.<number>} bytes Big endian byte representation
    	 * @param {boolean=} unsigned Whether unsigned or not, defaults to signed
    	 * @returns {Long} The corresponding Long value
    	 */
    	Long.fromBytesBE = function fromBytesBE(bytes, unsigned) {
    	    return new Long(
    	        bytes[4] << 24 |
    	        bytes[5] << 16 |
    	        bytes[6] <<  8 |
    	        bytes[7],
    	        bytes[0] << 24 |
    	        bytes[1] << 16 |
    	        bytes[2] <<  8 |
    	        bytes[3],
    	        unsigned
    	    );
    	};
    	return long;
    }

    var hasRequiredHelpers;

    function requireHelpers () {
    	if (hasRequiredHelpers) return helpers;
    	hasRequiredHelpers = 1;
    	/* eslint-disable */
    	/**
    	 * This file and any referenced files were automatically generated by @osmonauts/telescope@0.88.2
    	 * DO NOT MODIFY BY HAND. Instead, download the latest proto files for your chain
    	 * and run the transpile command or yarn proto command to regenerate this bundle.
    	 */
    	var __createBinding = (commonjsGlobal && commonjsGlobal.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    	    if (k2 === undefined) k2 = k;
    	    var desc = Object.getOwnPropertyDescriptor(m, k);
    	    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
    	      desc = { enumerable: true, get: function() { return m[k]; } };
    	    }
    	    Object.defineProperty(o, k2, desc);
    	}) : (function(o, m, k, k2) {
    	    if (k2 === undefined) k2 = k;
    	    o[k2] = m[k];
    	}));
    	var __setModuleDefault = (commonjsGlobal && commonjsGlobal.__setModuleDefault) || (Object.create ? (function(o, v) {
    	    Object.defineProperty(o, "default", { enumerable: true, value: v });
    	}) : function(o, v) {
    	    o["default"] = v;
    	});
    	var __importStar = (commonjsGlobal && commonjsGlobal.__importStar) || function (mod) {
    	    if (mod && mod.__esModule) return mod;
    	    var result = {};
    	    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    	    __setModuleDefault(result, mod);
    	    return result;
    	};
    	var __importDefault = (commonjsGlobal && commonjsGlobal.__importDefault) || function (mod) {
    	    return (mod && mod.__esModule) ? mod : { "default": mod };
    	};
    	Object.defineProperty(helpers, "__esModule", { value: true });
    	helpers.fromJsonTimestamp = helpers.fromTimestamp = helpers.toTimestamp = helpers.setPaginationParams = helpers.isObject = helpers.isSet = helpers.fromDuration = helpers.toDuration = helpers.omitDefault = helpers.base64FromBytes = helpers.bytesFromBase64 = helpers.Long = void 0;
    	const _m0 = __importStar(minimal);
    	const long_1 = __importDefault(requireLong());
    	helpers.Long = long_1.default;
    	// @ts-ignore
    	if (_m0.util.Long !== long_1.default) {
    	    _m0.util.Long = long_1.default;
    	    _m0.configure();
    	}
    	var globalThis = (() => {
    	    if (typeof globalThis !== "undefined")
    	        return globalThis;
    	    if (typeof self !== "undefined")
    	        return self;
    	    if (typeof window !== "undefined")
    	        return window;
    	    if (typeof commonjsGlobal !== "undefined")
    	        return commonjsGlobal;
    	    throw "Unable to locate global object";
    	})();
    	const atob = globalThis.atob || ((b64) => globalThis.Buffer.from(b64, "base64").toString("binary"));
    	function bytesFromBase64(b64) {
    	    const bin = atob(b64);
    	    const arr = new Uint8Array(bin.length);
    	    for (let i = 0; i < bin.length; ++i) {
    	        arr[i] = bin.charCodeAt(i);
    	    }
    	    return arr;
    	}
    	helpers.bytesFromBase64 = bytesFromBase64;
    	const btoa = globalThis.btoa || ((bin) => globalThis.Buffer.from(bin, "binary").toString("base64"));
    	function base64FromBytes(arr) {
    	    const bin = [];
    	    arr.forEach((byte) => {
    	        bin.push(String.fromCharCode(byte));
    	    });
    	    return btoa(bin.join(""));
    	}
    	helpers.base64FromBytes = base64FromBytes;
    	function omitDefault(input) {
    	    if (typeof input === "string") {
    	        return input === "" ? undefined : input;
    	    }
    	    if (typeof input === "number") {
    	        return input === 0 ? undefined : input;
    	    }
    	    if (long_1.default.isLong(input)) {
    	        return input.isZero() ? undefined : input;
    	    }
    	    throw new Error(`Got unsupported type ${typeof input}`);
    	}
    	helpers.omitDefault = omitDefault;
    	function toDuration(duration) {
    	    return {
    	        seconds: long_1.default.fromNumber(Math.floor(parseInt(duration) / 1000000000)),
    	        nanos: parseInt(duration) % 1000000000,
    	    };
    	}
    	helpers.toDuration = toDuration;
    	function fromDuration(duration) {
    	    return (parseInt(duration.seconds.toString()) * 1000000000 + duration.nanos).toString();
    	}
    	helpers.fromDuration = fromDuration;
    	function isSet(value) {
    	    return value !== null && value !== undefined;
    	}
    	helpers.isSet = isSet;
    	function isObject(value) {
    	    return typeof value === "object" && value !== null;
    	}
    	helpers.isObject = isObject;
    	const setPaginationParams = (options, pagination) => {
    	    if (!pagination) {
    	        return options;
    	    }
    	    if (typeof pagination?.countTotal !== "undefined") {
    	        options.params["pagination.count_total"] = pagination.countTotal;
    	    }
    	    if (typeof pagination?.key !== "undefined") {
    	        // String to Uint8Array
    	        // let uint8arr = new Uint8Array(Buffer.from(data,'base64'));
    	        // Uint8Array to String
    	        options.params["pagination.key"] = Buffer.from(pagination.key).toString("base64");
    	    }
    	    if (typeof pagination?.limit !== "undefined") {
    	        options.params["pagination.limit"] = pagination.limit.toString();
    	    }
    	    if (typeof pagination?.offset !== "undefined") {
    	        options.params["pagination.offset"] = pagination.offset.toString();
    	    }
    	    if (typeof pagination?.reverse !== "undefined") {
    	        options.params["pagination.reverse"] = pagination.reverse;
    	    }
    	    return options;
    	};
    	helpers.setPaginationParams = setPaginationParams;
    	function toTimestamp(date) {
    	    const seconds = numberToLong(date.getTime() / 1000);
    	    const nanos = (date.getTime() % 1000) * 1000000;
    	    return {
    	        seconds,
    	        nanos,
    	    };
    	}
    	helpers.toTimestamp = toTimestamp;
    	function fromTimestamp(t) {
    	    let millis = t.seconds.toNumber() * 1000;
    	    millis += t.nanos / 1000000;
    	    return new Date(millis);
    	}
    	helpers.fromTimestamp = fromTimestamp;
    	const timestampFromJSON = (object) => {
    	    return {
    	        seconds: isSet(object.seconds) ? long_1.default.fromValue(object.seconds) : long_1.default.ZERO,
    	        nanos: isSet(object.nanos) ? Number(object.nanos) : 0,
    	    };
    	};
    	function fromJsonTimestamp(o) {
    	    if (o instanceof Date) {
    	        return toTimestamp(o);
    	    }
    	    else if (typeof o === "string") {
    	        return toTimestamp(new Date(o));
    	    }
    	    else {
    	        return timestampFromJSON(o);
    	    }
    	}
    	helpers.fromJsonTimestamp = fromJsonTimestamp;
    	function numberToLong(number) {
    	    return long_1.default.fromNumber(number);
    	}
    	
    	return helpers;
    }

    var __createBinding$1 = (commonjsGlobal && commonjsGlobal.__createBinding) || (Object.create ? (function(o, m, k, k2) {
        if (k2 === undefined) k2 = k;
        var desc = Object.getOwnPropertyDescriptor(m, k);
        if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
          desc = { enumerable: true, get: function() { return m[k]; } };
        }
        Object.defineProperty(o, k2, desc);
    }) : (function(o, m, k, k2) {
        if (k2 === undefined) k2 = k;
        o[k2] = m[k];
    }));
    var __setModuleDefault$1 = (commonjsGlobal && commonjsGlobal.__setModuleDefault) || (Object.create ? (function(o, v) {
        Object.defineProperty(o, "default", { enumerable: true, value: v });
    }) : function(o, v) {
        o["default"] = v;
    });
    var __importStar$1 = (commonjsGlobal && commonjsGlobal.__importStar) || function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding$1(result, mod, k);
        __setModuleDefault$1(result, mod);
        return result;
    };
    Object.defineProperty(any, "__esModule", { value: true });
    any.Any = any.protobufPackage = void 0;
    /* eslint-disable */
    const _m0$1 = __importStar$1(minimal);
    const helpers_1$1 = requireHelpers();
    any.protobufPackage = "google.protobuf";
    function createBaseAny() {
        return {
            typeUrl: "",
            value: new Uint8Array(),
        };
    }
    any.Any = {
        encode(message, writer = _m0$1.Writer.create()) {
            if (message.typeUrl !== "") {
                writer.uint32(10).string(message.typeUrl);
            }
            if (message.value.length !== 0) {
                writer.uint32(18).bytes(message.value);
            }
            return writer;
        },
        decode(input, length) {
            const reader = input instanceof _m0$1.Reader ? input : new _m0$1.Reader(input);
            let end = length === undefined ? reader.len : reader.pos + length;
            const message = createBaseAny();
            while (reader.pos < end) {
                const tag = reader.uint32();
                switch (tag >>> 3) {
                    case 1:
                        message.typeUrl = reader.string();
                        break;
                    case 2:
                        message.value = reader.bytes();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                }
            }
            return message;
        },
        fromJSON(object) {
            return {
                typeUrl: (0, helpers_1$1.isSet)(object.typeUrl) ? String(object.typeUrl) : "",
                value: (0, helpers_1$1.isSet)(object.value) ? (0, helpers_1$1.bytesFromBase64)(object.value) : new Uint8Array(),
            };
        },
        toJSON(message) {
            const obj = {};
            message.typeUrl !== undefined && (obj.typeUrl = message.typeUrl);
            message.value !== undefined &&
                (obj.value = (0, helpers_1$1.base64FromBytes)(message.value !== undefined ? message.value : new Uint8Array()));
            return obj;
        },
        fromPartial(object) {
            const message = createBaseAny();
            message.typeUrl = object.typeUrl ?? "";
            message.value = object.value ?? new Uint8Array();
            return message;
        },
    };

    (function (exports) {
    	var __createBinding = (commonjsGlobal && commonjsGlobal.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    	    if (k2 === undefined) k2 = k;
    	    var desc = Object.getOwnPropertyDescriptor(m, k);
    	    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
    	      desc = { enumerable: true, get: function() { return m[k]; } };
    	    }
    	    Object.defineProperty(o, k2, desc);
    	}) : (function(o, m, k, k2) {
    	    if (k2 === undefined) k2 = k;
    	    o[k2] = m[k];
    	}));
    	var __setModuleDefault = (commonjsGlobal && commonjsGlobal.__setModuleDefault) || (Object.create ? (function(o, v) {
    	    Object.defineProperty(o, "default", { enumerable: true, value: v });
    	}) : function(o, v) {
    	    o["default"] = v;
    	});
    	var __importStar = (commonjsGlobal && commonjsGlobal.__importStar) || function (mod) {
    	    if (mod && mod.__esModule) return mod;
    	    var result = {};
    	    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    	    __setModuleDefault(result, mod);
    	    return result;
    	};
    	Object.defineProperty(exports, "__esModule", { value: true });
    	exports.Model = exports.AbsoluteTxPosition = exports.ContractCodeHistoryEntry = exports.ContractInfo = exports.CodeInfo = exports.Params = exports.AccessConfig = exports.AccessTypeParam = exports.contractCodeHistoryOperationTypeToJSON = exports.contractCodeHistoryOperationTypeFromJSON = exports.ContractCodeHistoryOperationType = exports.accessTypeToJSON = exports.accessTypeFromJSON = exports.AccessType = exports.protobufPackage = void 0;
    	/* eslint-disable */
    	const any_1 = any;
    	const _m0 = __importStar(minimal);
    	const helpers_1 = requireHelpers();
    	exports.protobufPackage = "cosmwasm.wasm.v1";
    	/** AccessType permission types */
    	var AccessType;
    	(function (AccessType) {
    	    /** ACCESS_TYPE_UNSPECIFIED - AccessTypeUnspecified placeholder for empty value */
    	    AccessType[AccessType["ACCESS_TYPE_UNSPECIFIED"] = 0] = "ACCESS_TYPE_UNSPECIFIED";
    	    /** ACCESS_TYPE_NOBODY - AccessTypeNobody forbidden */
    	    AccessType[AccessType["ACCESS_TYPE_NOBODY"] = 1] = "ACCESS_TYPE_NOBODY";
    	    /**
    	     * ACCESS_TYPE_ONLY_ADDRESS - AccessTypeOnlyAddress restricted to a single address
    	     * Deprecated: use AccessTypeAnyOfAddresses instead
    	     */
    	    AccessType[AccessType["ACCESS_TYPE_ONLY_ADDRESS"] = 2] = "ACCESS_TYPE_ONLY_ADDRESS";
    	    /** ACCESS_TYPE_EVERYBODY - AccessTypeEverybody unrestricted */
    	    AccessType[AccessType["ACCESS_TYPE_EVERYBODY"] = 3] = "ACCESS_TYPE_EVERYBODY";
    	    /** ACCESS_TYPE_ANY_OF_ADDRESSES - AccessTypeAnyOfAddresses allow any of the addresses */
    	    AccessType[AccessType["ACCESS_TYPE_ANY_OF_ADDRESSES"] = 4] = "ACCESS_TYPE_ANY_OF_ADDRESSES";
    	    AccessType[AccessType["UNRECOGNIZED"] = -1] = "UNRECOGNIZED";
    	})(AccessType = exports.AccessType || (exports.AccessType = {}));
    	function accessTypeFromJSON(object) {
    	    switch (object) {
    	        case 0:
    	        case "ACCESS_TYPE_UNSPECIFIED":
    	            return AccessType.ACCESS_TYPE_UNSPECIFIED;
    	        case 1:
    	        case "ACCESS_TYPE_NOBODY":
    	            return AccessType.ACCESS_TYPE_NOBODY;
    	        case 2:
    	        case "ACCESS_TYPE_ONLY_ADDRESS":
    	            return AccessType.ACCESS_TYPE_ONLY_ADDRESS;
    	        case 3:
    	        case "ACCESS_TYPE_EVERYBODY":
    	            return AccessType.ACCESS_TYPE_EVERYBODY;
    	        case 4:
    	        case "ACCESS_TYPE_ANY_OF_ADDRESSES":
    	            return AccessType.ACCESS_TYPE_ANY_OF_ADDRESSES;
    	        case -1:
    	        case "UNRECOGNIZED":
    	        default:
    	            return AccessType.UNRECOGNIZED;
    	    }
    	}
    	exports.accessTypeFromJSON = accessTypeFromJSON;
    	function accessTypeToJSON(object) {
    	    switch (object) {
    	        case AccessType.ACCESS_TYPE_UNSPECIFIED:
    	            return "ACCESS_TYPE_UNSPECIFIED";
    	        case AccessType.ACCESS_TYPE_NOBODY:
    	            return "ACCESS_TYPE_NOBODY";
    	        case AccessType.ACCESS_TYPE_ONLY_ADDRESS:
    	            return "ACCESS_TYPE_ONLY_ADDRESS";
    	        case AccessType.ACCESS_TYPE_EVERYBODY:
    	            return "ACCESS_TYPE_EVERYBODY";
    	        case AccessType.ACCESS_TYPE_ANY_OF_ADDRESSES:
    	            return "ACCESS_TYPE_ANY_OF_ADDRESSES";
    	        case AccessType.UNRECOGNIZED:
    	        default:
    	            return "UNRECOGNIZED";
    	    }
    	}
    	exports.accessTypeToJSON = accessTypeToJSON;
    	/** ContractCodeHistoryOperationType actions that caused a code change */
    	var ContractCodeHistoryOperationType;
    	(function (ContractCodeHistoryOperationType) {
    	    /** CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED - ContractCodeHistoryOperationTypeUnspecified placeholder for empty value */
    	    ContractCodeHistoryOperationType[ContractCodeHistoryOperationType["CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED"] = 0] = "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED";
    	    /** CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT - ContractCodeHistoryOperationTypeInit on chain contract instantiation */
    	    ContractCodeHistoryOperationType[ContractCodeHistoryOperationType["CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT"] = 1] = "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT";
    	    /** CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE - ContractCodeHistoryOperationTypeMigrate code migration */
    	    ContractCodeHistoryOperationType[ContractCodeHistoryOperationType["CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE"] = 2] = "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE";
    	    /** CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS - ContractCodeHistoryOperationTypeGenesis based on genesis data */
    	    ContractCodeHistoryOperationType[ContractCodeHistoryOperationType["CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS"] = 3] = "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS";
    	    ContractCodeHistoryOperationType[ContractCodeHistoryOperationType["UNRECOGNIZED"] = -1] = "UNRECOGNIZED";
    	})(ContractCodeHistoryOperationType = exports.ContractCodeHistoryOperationType || (exports.ContractCodeHistoryOperationType = {}));
    	function contractCodeHistoryOperationTypeFromJSON(object) {
    	    switch (object) {
    	        case 0:
    	        case "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED":
    	            return ContractCodeHistoryOperationType.CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED;
    	        case 1:
    	        case "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT":
    	            return ContractCodeHistoryOperationType.CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT;
    	        case 2:
    	        case "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE":
    	            return ContractCodeHistoryOperationType.CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE;
    	        case 3:
    	        case "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS":
    	            return ContractCodeHistoryOperationType.CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS;
    	        case -1:
    	        case "UNRECOGNIZED":
    	        default:
    	            return ContractCodeHistoryOperationType.UNRECOGNIZED;
    	    }
    	}
    	exports.contractCodeHistoryOperationTypeFromJSON = contractCodeHistoryOperationTypeFromJSON;
    	function contractCodeHistoryOperationTypeToJSON(object) {
    	    switch (object) {
    	        case ContractCodeHistoryOperationType.CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED:
    	            return "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED";
    	        case ContractCodeHistoryOperationType.CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT:
    	            return "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT";
    	        case ContractCodeHistoryOperationType.CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE:
    	            return "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE";
    	        case ContractCodeHistoryOperationType.CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS:
    	            return "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS";
    	        case ContractCodeHistoryOperationType.UNRECOGNIZED:
    	        default:
    	            return "UNRECOGNIZED";
    	    }
    	}
    	exports.contractCodeHistoryOperationTypeToJSON = contractCodeHistoryOperationTypeToJSON;
    	function createBaseAccessTypeParam() {
    	    return {
    	        value: 0,
    	    };
    	}
    	exports.AccessTypeParam = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.value !== 0) {
    	            writer.uint32(8).int32(message.value);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseAccessTypeParam();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.value = reader.int32();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            value: (0, helpers_1.isSet)(object.value) ? accessTypeFromJSON(object.value) : 0,
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.value !== undefined && (obj.value = accessTypeToJSON(message.value));
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseAccessTypeParam();
    	        message.value = object.value ?? 0;
    	        return message;
    	    },
    	};
    	function createBaseAccessConfig() {
    	    return {
    	        permission: 0,
    	        address: "",
    	        addresses: [],
    	    };
    	}
    	exports.AccessConfig = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.permission !== 0) {
    	            writer.uint32(8).int32(message.permission);
    	        }
    	        if (message.address !== "") {
    	            writer.uint32(18).string(message.address);
    	        }
    	        for (const v of message.addresses) {
    	            writer.uint32(26).string(v);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseAccessConfig();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.permission = reader.int32();
    	                    break;
    	                case 2:
    	                    message.address = reader.string();
    	                    break;
    	                case 3:
    	                    message.addresses.push(reader.string());
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            permission: (0, helpers_1.isSet)(object.permission) ? accessTypeFromJSON(object.permission) : 0,
    	            address: (0, helpers_1.isSet)(object.address) ? String(object.address) : "",
    	            addresses: Array.isArray(object?.addresses) ? object.addresses.map((e) => String(e)) : [],
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.permission !== undefined && (obj.permission = accessTypeToJSON(message.permission));
    	        message.address !== undefined && (obj.address = message.address);
    	        if (message.addresses) {
    	            obj.addresses = message.addresses.map((e) => e);
    	        }
    	        else {
    	            obj.addresses = [];
    	        }
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseAccessConfig();
    	        message.permission = object.permission ?? 0;
    	        message.address = object.address ?? "";
    	        message.addresses = object.addresses?.map((e) => e) || [];
    	        return message;
    	    },
    	};
    	function createBaseParams() {
    	    return {
    	        codeUploadAccess: undefined,
    	        instantiateDefaultPermission: 0,
    	    };
    	}
    	exports.Params = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.codeUploadAccess !== undefined) {
    	            exports.AccessConfig.encode(message.codeUploadAccess, writer.uint32(10).fork()).ldelim();
    	        }
    	        if (message.instantiateDefaultPermission !== 0) {
    	            writer.uint32(16).int32(message.instantiateDefaultPermission);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseParams();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.codeUploadAccess = exports.AccessConfig.decode(reader, reader.uint32());
    	                    break;
    	                case 2:
    	                    message.instantiateDefaultPermission = reader.int32();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            codeUploadAccess: (0, helpers_1.isSet)(object.codeUploadAccess)
    	                ? exports.AccessConfig.fromJSON(object.codeUploadAccess)
    	                : undefined,
    	            instantiateDefaultPermission: (0, helpers_1.isSet)(object.instantiateDefaultPermission)
    	                ? accessTypeFromJSON(object.instantiateDefaultPermission)
    	                : 0,
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.codeUploadAccess !== undefined &&
    	            (obj.codeUploadAccess = message.codeUploadAccess
    	                ? exports.AccessConfig.toJSON(message.codeUploadAccess)
    	                : undefined);
    	        message.instantiateDefaultPermission !== undefined &&
    	            (obj.instantiateDefaultPermission = accessTypeToJSON(message.instantiateDefaultPermission));
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseParams();
    	        message.codeUploadAccess =
    	            object.codeUploadAccess !== undefined && object.codeUploadAccess !== null
    	                ? exports.AccessConfig.fromPartial(object.codeUploadAccess)
    	                : undefined;
    	        message.instantiateDefaultPermission = object.instantiateDefaultPermission ?? 0;
    	        return message;
    	    },
    	};
    	function createBaseCodeInfo() {
    	    return {
    	        codeHash: new Uint8Array(),
    	        creator: "",
    	        instantiateConfig: undefined,
    	    };
    	}
    	exports.CodeInfo = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.codeHash.length !== 0) {
    	            writer.uint32(10).bytes(message.codeHash);
    	        }
    	        if (message.creator !== "") {
    	            writer.uint32(18).string(message.creator);
    	        }
    	        if (message.instantiateConfig !== undefined) {
    	            exports.AccessConfig.encode(message.instantiateConfig, writer.uint32(42).fork()).ldelim();
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseCodeInfo();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.codeHash = reader.bytes();
    	                    break;
    	                case 2:
    	                    message.creator = reader.string();
    	                    break;
    	                case 5:
    	                    message.instantiateConfig = exports.AccessConfig.decode(reader, reader.uint32());
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            codeHash: (0, helpers_1.isSet)(object.codeHash) ? (0, helpers_1.bytesFromBase64)(object.codeHash) : new Uint8Array(),
    	            creator: (0, helpers_1.isSet)(object.creator) ? String(object.creator) : "",
    	            instantiateConfig: (0, helpers_1.isSet)(object.instantiateConfig)
    	                ? exports.AccessConfig.fromJSON(object.instantiateConfig)
    	                : undefined,
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.codeHash !== undefined &&
    	            (obj.codeHash = (0, helpers_1.base64FromBytes)(message.codeHash !== undefined ? message.codeHash : new Uint8Array()));
    	        message.creator !== undefined && (obj.creator = message.creator);
    	        message.instantiateConfig !== undefined &&
    	            (obj.instantiateConfig = message.instantiateConfig
    	                ? exports.AccessConfig.toJSON(message.instantiateConfig)
    	                : undefined);
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseCodeInfo();
    	        message.codeHash = object.codeHash ?? new Uint8Array();
    	        message.creator = object.creator ?? "";
    	        message.instantiateConfig =
    	            object.instantiateConfig !== undefined && object.instantiateConfig !== null
    	                ? exports.AccessConfig.fromPartial(object.instantiateConfig)
    	                : undefined;
    	        return message;
    	    },
    	};
    	function createBaseContractInfo() {
    	    return {
    	        codeId: helpers_1.Long.UZERO,
    	        creator: "",
    	        admin: "",
    	        label: "",
    	        created: undefined,
    	        ibcPortId: "",
    	        extension: undefined,
    	    };
    	}
    	exports.ContractInfo = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (!message.codeId.isZero()) {
    	            writer.uint32(8).uint64(message.codeId);
    	        }
    	        if (message.creator !== "") {
    	            writer.uint32(18).string(message.creator);
    	        }
    	        if (message.admin !== "") {
    	            writer.uint32(26).string(message.admin);
    	        }
    	        if (message.label !== "") {
    	            writer.uint32(34).string(message.label);
    	        }
    	        if (message.created !== undefined) {
    	            exports.AbsoluteTxPosition.encode(message.created, writer.uint32(42).fork()).ldelim();
    	        }
    	        if (message.ibcPortId !== "") {
    	            writer.uint32(50).string(message.ibcPortId);
    	        }
    	        if (message.extension !== undefined) {
    	            any_1.Any.encode(message.extension, writer.uint32(58).fork()).ldelim();
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseContractInfo();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.codeId = reader.uint64();
    	                    break;
    	                case 2:
    	                    message.creator = reader.string();
    	                    break;
    	                case 3:
    	                    message.admin = reader.string();
    	                    break;
    	                case 4:
    	                    message.label = reader.string();
    	                    break;
    	                case 5:
    	                    message.created = exports.AbsoluteTxPosition.decode(reader, reader.uint32());
    	                    break;
    	                case 6:
    	                    message.ibcPortId = reader.string();
    	                    break;
    	                case 7:
    	                    message.extension = any_1.Any.decode(reader, reader.uint32());
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            codeId: (0, helpers_1.isSet)(object.codeId) ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO,
    	            creator: (0, helpers_1.isSet)(object.creator) ? String(object.creator) : "",
    	            admin: (0, helpers_1.isSet)(object.admin) ? String(object.admin) : "",
    	            label: (0, helpers_1.isSet)(object.label) ? String(object.label) : "",
    	            created: (0, helpers_1.isSet)(object.created) ? exports.AbsoluteTxPosition.fromJSON(object.created) : undefined,
    	            ibcPortId: (0, helpers_1.isSet)(object.ibcPortId) ? String(object.ibcPortId) : "",
    	            extension: (0, helpers_1.isSet)(object.extension) ? any_1.Any.fromJSON(object.extension) : undefined,
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.codeId !== undefined && (obj.codeId = (message.codeId || helpers_1.Long.UZERO).toString());
    	        message.creator !== undefined && (obj.creator = message.creator);
    	        message.admin !== undefined && (obj.admin = message.admin);
    	        message.label !== undefined && (obj.label = message.label);
    	        message.created !== undefined &&
    	            (obj.created = message.created ? exports.AbsoluteTxPosition.toJSON(message.created) : undefined);
    	        message.ibcPortId !== undefined && (obj.ibcPortId = message.ibcPortId);
    	        message.extension !== undefined &&
    	            (obj.extension = message.extension ? any_1.Any.toJSON(message.extension) : undefined);
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseContractInfo();
    	        message.codeId =
    	            object.codeId !== undefined && object.codeId !== null ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO;
    	        message.creator = object.creator ?? "";
    	        message.admin = object.admin ?? "";
    	        message.label = object.label ?? "";
    	        message.created =
    	            object.created !== undefined && object.created !== null
    	                ? exports.AbsoluteTxPosition.fromPartial(object.created)
    	                : undefined;
    	        message.ibcPortId = object.ibcPortId ?? "";
    	        message.extension =
    	            object.extension !== undefined && object.extension !== null
    	                ? any_1.Any.fromPartial(object.extension)
    	                : undefined;
    	        return message;
    	    },
    	};
    	function createBaseContractCodeHistoryEntry() {
    	    return {
    	        operation: 0,
    	        codeId: helpers_1.Long.UZERO,
    	        updated: undefined,
    	        msg: new Uint8Array(),
    	    };
    	}
    	exports.ContractCodeHistoryEntry = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.operation !== 0) {
    	            writer.uint32(8).int32(message.operation);
    	        }
    	        if (!message.codeId.isZero()) {
    	            writer.uint32(16).uint64(message.codeId);
    	        }
    	        if (message.updated !== undefined) {
    	            exports.AbsoluteTxPosition.encode(message.updated, writer.uint32(26).fork()).ldelim();
    	        }
    	        if (message.msg.length !== 0) {
    	            writer.uint32(34).bytes(message.msg);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseContractCodeHistoryEntry();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.operation = reader.int32();
    	                    break;
    	                case 2:
    	                    message.codeId = reader.uint64();
    	                    break;
    	                case 3:
    	                    message.updated = exports.AbsoluteTxPosition.decode(reader, reader.uint32());
    	                    break;
    	                case 4:
    	                    message.msg = reader.bytes();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            operation: (0, helpers_1.isSet)(object.operation) ? contractCodeHistoryOperationTypeFromJSON(object.operation) : 0,
    	            codeId: (0, helpers_1.isSet)(object.codeId) ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO,
    	            updated: (0, helpers_1.isSet)(object.updated) ? exports.AbsoluteTxPosition.fromJSON(object.updated) : undefined,
    	            msg: (0, helpers_1.isSet)(object.msg) ? (0, helpers_1.bytesFromBase64)(object.msg) : new Uint8Array(),
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.operation !== undefined &&
    	            (obj.operation = contractCodeHistoryOperationTypeToJSON(message.operation));
    	        message.codeId !== undefined && (obj.codeId = (message.codeId || helpers_1.Long.UZERO).toString());
    	        message.updated !== undefined &&
    	            (obj.updated = message.updated ? exports.AbsoluteTxPosition.toJSON(message.updated) : undefined);
    	        message.msg !== undefined &&
    	            (obj.msg = (0, helpers_1.base64FromBytes)(message.msg !== undefined ? message.msg : new Uint8Array()));
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseContractCodeHistoryEntry();
    	        message.operation = object.operation ?? 0;
    	        message.codeId =
    	            object.codeId !== undefined && object.codeId !== null ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO;
    	        message.updated =
    	            object.updated !== undefined && object.updated !== null
    	                ? exports.AbsoluteTxPosition.fromPartial(object.updated)
    	                : undefined;
    	        message.msg = object.msg ?? new Uint8Array();
    	        return message;
    	    },
    	};
    	function createBaseAbsoluteTxPosition() {
    	    return {
    	        blockHeight: helpers_1.Long.UZERO,
    	        txIndex: helpers_1.Long.UZERO,
    	    };
    	}
    	exports.AbsoluteTxPosition = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (!message.blockHeight.isZero()) {
    	            writer.uint32(8).uint64(message.blockHeight);
    	        }
    	        if (!message.txIndex.isZero()) {
    	            writer.uint32(16).uint64(message.txIndex);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseAbsoluteTxPosition();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.blockHeight = reader.uint64();
    	                    break;
    	                case 2:
    	                    message.txIndex = reader.uint64();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            blockHeight: (0, helpers_1.isSet)(object.blockHeight) ? helpers_1.Long.fromValue(object.blockHeight) : helpers_1.Long.UZERO,
    	            txIndex: (0, helpers_1.isSet)(object.txIndex) ? helpers_1.Long.fromValue(object.txIndex) : helpers_1.Long.UZERO,
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.blockHeight !== undefined && (obj.blockHeight = (message.blockHeight || helpers_1.Long.UZERO).toString());
    	        message.txIndex !== undefined && (obj.txIndex = (message.txIndex || helpers_1.Long.UZERO).toString());
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseAbsoluteTxPosition();
    	        message.blockHeight =
    	            object.blockHeight !== undefined && object.blockHeight !== null
    	                ? helpers_1.Long.fromValue(object.blockHeight)
    	                : helpers_1.Long.UZERO;
    	        message.txIndex =
    	            object.txIndex !== undefined && object.txIndex !== null ? helpers_1.Long.fromValue(object.txIndex) : helpers_1.Long.UZERO;
    	        return message;
    	    },
    	};
    	function createBaseModel() {
    	    return {
    	        key: new Uint8Array(),
    	        value: new Uint8Array(),
    	    };
    	}
    	exports.Model = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.key.length !== 0) {
    	            writer.uint32(10).bytes(message.key);
    	        }
    	        if (message.value.length !== 0) {
    	            writer.uint32(18).bytes(message.value);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseModel();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.key = reader.bytes();
    	                    break;
    	                case 2:
    	                    message.value = reader.bytes();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            key: (0, helpers_1.isSet)(object.key) ? (0, helpers_1.bytesFromBase64)(object.key) : new Uint8Array(),
    	            value: (0, helpers_1.isSet)(object.value) ? (0, helpers_1.bytesFromBase64)(object.value) : new Uint8Array(),
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.key !== undefined &&
    	            (obj.key = (0, helpers_1.base64FromBytes)(message.key !== undefined ? message.key : new Uint8Array()));
    	        message.value !== undefined &&
    	            (obj.value = (0, helpers_1.base64FromBytes)(message.value !== undefined ? message.value : new Uint8Array()));
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseModel();
    	        message.key = object.key ?? new Uint8Array();
    	        message.value = object.value ?? new Uint8Array();
    	        return message;
    	    },
    	};
    	
    } (types));

    var coin = {};

    var __createBinding = (commonjsGlobal && commonjsGlobal.__createBinding) || (Object.create ? (function(o, m, k, k2) {
        if (k2 === undefined) k2 = k;
        var desc = Object.getOwnPropertyDescriptor(m, k);
        if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
          desc = { enumerable: true, get: function() { return m[k]; } };
        }
        Object.defineProperty(o, k2, desc);
    }) : (function(o, m, k, k2) {
        if (k2 === undefined) k2 = k;
        o[k2] = m[k];
    }));
    var __setModuleDefault = (commonjsGlobal && commonjsGlobal.__setModuleDefault) || (Object.create ? (function(o, v) {
        Object.defineProperty(o, "default", { enumerable: true, value: v });
    }) : function(o, v) {
        o["default"] = v;
    });
    var __importStar = (commonjsGlobal && commonjsGlobal.__importStar) || function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
        __setModuleDefault(result, mod);
        return result;
    };
    Object.defineProperty(coin, "__esModule", { value: true });
    coin.DecProto = coin.IntProto = coin.DecCoin = coin.Coin = coin.protobufPackage = void 0;
    /* eslint-disable */
    const _m0 = __importStar(minimal);
    const helpers_1 = requireHelpers();
    coin.protobufPackage = "cosmos.base.v1beta1";
    function createBaseCoin() {
        return {
            denom: "",
            amount: "",
        };
    }
    coin.Coin = {
        encode(message, writer = _m0.Writer.create()) {
            if (message.denom !== "") {
                writer.uint32(10).string(message.denom);
            }
            if (message.amount !== "") {
                writer.uint32(18).string(message.amount);
            }
            return writer;
        },
        decode(input, length) {
            const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
            let end = length === undefined ? reader.len : reader.pos + length;
            const message = createBaseCoin();
            while (reader.pos < end) {
                const tag = reader.uint32();
                switch (tag >>> 3) {
                    case 1:
                        message.denom = reader.string();
                        break;
                    case 2:
                        message.amount = reader.string();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                }
            }
            return message;
        },
        fromJSON(object) {
            return {
                denom: (0, helpers_1.isSet)(object.denom) ? String(object.denom) : "",
                amount: (0, helpers_1.isSet)(object.amount) ? String(object.amount) : "",
            };
        },
        toJSON(message) {
            const obj = {};
            message.denom !== undefined && (obj.denom = message.denom);
            message.amount !== undefined && (obj.amount = message.amount);
            return obj;
        },
        fromPartial(object) {
            const message = createBaseCoin();
            message.denom = object.denom ?? "";
            message.amount = object.amount ?? "";
            return message;
        },
    };
    function createBaseDecCoin() {
        return {
            denom: "",
            amount: "",
        };
    }
    coin.DecCoin = {
        encode(message, writer = _m0.Writer.create()) {
            if (message.denom !== "") {
                writer.uint32(10).string(message.denom);
            }
            if (message.amount !== "") {
                writer.uint32(18).string(message.amount);
            }
            return writer;
        },
        decode(input, length) {
            const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
            let end = length === undefined ? reader.len : reader.pos + length;
            const message = createBaseDecCoin();
            while (reader.pos < end) {
                const tag = reader.uint32();
                switch (tag >>> 3) {
                    case 1:
                        message.denom = reader.string();
                        break;
                    case 2:
                        message.amount = reader.string();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                }
            }
            return message;
        },
        fromJSON(object) {
            return {
                denom: (0, helpers_1.isSet)(object.denom) ? String(object.denom) : "",
                amount: (0, helpers_1.isSet)(object.amount) ? String(object.amount) : "",
            };
        },
        toJSON(message) {
            const obj = {};
            message.denom !== undefined && (obj.denom = message.denom);
            message.amount !== undefined && (obj.amount = message.amount);
            return obj;
        },
        fromPartial(object) {
            const message = createBaseDecCoin();
            message.denom = object.denom ?? "";
            message.amount = object.amount ?? "";
            return message;
        },
    };
    function createBaseIntProto() {
        return {
            int: "",
        };
    }
    coin.IntProto = {
        encode(message, writer = _m0.Writer.create()) {
            if (message.int !== "") {
                writer.uint32(10).string(message.int);
            }
            return writer;
        },
        decode(input, length) {
            const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
            let end = length === undefined ? reader.len : reader.pos + length;
            const message = createBaseIntProto();
            while (reader.pos < end) {
                const tag = reader.uint32();
                switch (tag >>> 3) {
                    case 1:
                        message.int = reader.string();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                }
            }
            return message;
        },
        fromJSON(object) {
            return {
                int: (0, helpers_1.isSet)(object.int) ? String(object.int) : "",
            };
        },
        toJSON(message) {
            const obj = {};
            message.int !== undefined && (obj.int = message.int);
            return obj;
        },
        fromPartial(object) {
            const message = createBaseIntProto();
            message.int = object.int ?? "";
            return message;
        },
    };
    function createBaseDecProto() {
        return {
            dec: "",
        };
    }
    coin.DecProto = {
        encode(message, writer = _m0.Writer.create()) {
            if (message.dec !== "") {
                writer.uint32(10).string(message.dec);
            }
            return writer;
        },
        decode(input, length) {
            const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
            let end = length === undefined ? reader.len : reader.pos + length;
            const message = createBaseDecProto();
            while (reader.pos < end) {
                const tag = reader.uint32();
                switch (tag >>> 3) {
                    case 1:
                        message.dec = reader.string();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                }
            }
            return message;
        },
        fromJSON(object) {
            return {
                dec: (0, helpers_1.isSet)(object.dec) ? String(object.dec) : "",
            };
        },
        toJSON(message) {
            const obj = {};
            message.dec !== undefined && (obj.dec = message.dec);
            return obj;
        },
        fromPartial(object) {
            const message = createBaseDecProto();
            message.dec = object.dec ?? "";
            return message;
        },
    };

    (function (exports) {
    	var __createBinding = (commonjsGlobal && commonjsGlobal.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    	    if (k2 === undefined) k2 = k;
    	    var desc = Object.getOwnPropertyDescriptor(m, k);
    	    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
    	      desc = { enumerable: true, get: function() { return m[k]; } };
    	    }
    	    Object.defineProperty(o, k2, desc);
    	}) : (function(o, m, k, k2) {
    	    if (k2 === undefined) k2 = k;
    	    o[k2] = m[k];
    	}));
    	var __setModuleDefault = (commonjsGlobal && commonjsGlobal.__setModuleDefault) || (Object.create ? (function(o, v) {
    	    Object.defineProperty(o, "default", { enumerable: true, value: v });
    	}) : function(o, v) {
    	    o["default"] = v;
    	});
    	var __importStar = (commonjsGlobal && commonjsGlobal.__importStar) || function (mod) {
    	    if (mod && mod.__esModule) return mod;
    	    var result = {};
    	    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    	    __setModuleDefault(result, mod);
    	    return result;
    	};
    	Object.defineProperty(exports, "__esModule", { value: true });
    	exports.MsgClientImpl = exports.MsgUpdateInstantiateConfigResponse = exports.MsgUpdateInstantiateConfig = exports.MsgClearAdminResponse = exports.MsgClearAdmin = exports.MsgUpdateAdminResponse = exports.MsgUpdateAdmin = exports.MsgMigrateContractResponse = exports.MsgMigrateContract = exports.MsgExecuteContractResponse = exports.MsgExecuteContract = exports.MsgInstantiateContract2Response = exports.MsgInstantiateContractResponse = exports.MsgInstantiateContract2 = exports.MsgInstantiateContract = exports.MsgStoreCodeResponse = exports.MsgStoreCode = exports.protobufPackage = void 0;
    	/* eslint-disable */
    	const types_1 = types;
    	const coin_1 = coin;
    	const _m0 = __importStar(minimal);
    	const helpers_1 = requireHelpers();
    	exports.protobufPackage = "cosmwasm.wasm.v1";
    	function createBaseMsgStoreCode() {
    	    return {
    	        sender: "",
    	        wasmByteCode: new Uint8Array(),
    	        instantiatePermission: undefined,
    	    };
    	}
    	exports.MsgStoreCode = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.sender !== "") {
    	            writer.uint32(10).string(message.sender);
    	        }
    	        if (message.wasmByteCode.length !== 0) {
    	            writer.uint32(18).bytes(message.wasmByteCode);
    	        }
    	        if (message.instantiatePermission !== undefined) {
    	            types_1.AccessConfig.encode(message.instantiatePermission, writer.uint32(42).fork()).ldelim();
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgStoreCode();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.sender = reader.string();
    	                    break;
    	                case 2:
    	                    message.wasmByteCode = reader.bytes();
    	                    break;
    	                case 5:
    	                    message.instantiatePermission = types_1.AccessConfig.decode(reader, reader.uint32());
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            sender: (0, helpers_1.isSet)(object.sender) ? String(object.sender) : "",
    	            wasmByteCode: (0, helpers_1.isSet)(object.wasmByteCode) ? (0, helpers_1.bytesFromBase64)(object.wasmByteCode) : new Uint8Array(),
    	            instantiatePermission: (0, helpers_1.isSet)(object.instantiatePermission)
    	                ? types_1.AccessConfig.fromJSON(object.instantiatePermission)
    	                : undefined,
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.sender !== undefined && (obj.sender = message.sender);
    	        message.wasmByteCode !== undefined &&
    	            (obj.wasmByteCode = (0, helpers_1.base64FromBytes)(message.wasmByteCode !== undefined ? message.wasmByteCode : new Uint8Array()));
    	        message.instantiatePermission !== undefined &&
    	            (obj.instantiatePermission = message.instantiatePermission
    	                ? types_1.AccessConfig.toJSON(message.instantiatePermission)
    	                : undefined);
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseMsgStoreCode();
    	        message.sender = object.sender ?? "";
    	        message.wasmByteCode = object.wasmByteCode ?? new Uint8Array();
    	        message.instantiatePermission =
    	            object.instantiatePermission !== undefined && object.instantiatePermission !== null
    	                ? types_1.AccessConfig.fromPartial(object.instantiatePermission)
    	                : undefined;
    	        return message;
    	    },
    	};
    	function createBaseMsgStoreCodeResponse() {
    	    return {
    	        codeId: helpers_1.Long.UZERO,
    	        checksum: new Uint8Array(),
    	    };
    	}
    	exports.MsgStoreCodeResponse = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (!message.codeId.isZero()) {
    	            writer.uint32(8).uint64(message.codeId);
    	        }
    	        if (message.checksum.length !== 0) {
    	            writer.uint32(18).bytes(message.checksum);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgStoreCodeResponse();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.codeId = reader.uint64();
    	                    break;
    	                case 2:
    	                    message.checksum = reader.bytes();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            codeId: (0, helpers_1.isSet)(object.codeId) ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO,
    	            checksum: (0, helpers_1.isSet)(object.checksum) ? (0, helpers_1.bytesFromBase64)(object.checksum) : new Uint8Array(),
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.codeId !== undefined && (obj.codeId = (message.codeId || helpers_1.Long.UZERO).toString());
    	        message.checksum !== undefined &&
    	            (obj.checksum = (0, helpers_1.base64FromBytes)(message.checksum !== undefined ? message.checksum : new Uint8Array()));
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseMsgStoreCodeResponse();
    	        message.codeId =
    	            object.codeId !== undefined && object.codeId !== null ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO;
    	        message.checksum = object.checksum ?? new Uint8Array();
    	        return message;
    	    },
    	};
    	function createBaseMsgInstantiateContract() {
    	    return {
    	        sender: "",
    	        admin: "",
    	        codeId: helpers_1.Long.UZERO,
    	        label: "",
    	        msg: new Uint8Array(),
    	        funds: [],
    	    };
    	}
    	exports.MsgInstantiateContract = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.sender !== "") {
    	            writer.uint32(10).string(message.sender);
    	        }
    	        if (message.admin !== "") {
    	            writer.uint32(18).string(message.admin);
    	        }
    	        if (!message.codeId.isZero()) {
    	            writer.uint32(24).uint64(message.codeId);
    	        }
    	        if (message.label !== "") {
    	            writer.uint32(34).string(message.label);
    	        }
    	        if (message.msg.length !== 0) {
    	            writer.uint32(42).bytes(message.msg);
    	        }
    	        for (const v of message.funds) {
    	            coin_1.Coin.encode(v, writer.uint32(50).fork()).ldelim();
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgInstantiateContract();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.sender = reader.string();
    	                    break;
    	                case 2:
    	                    message.admin = reader.string();
    	                    break;
    	                case 3:
    	                    message.codeId = reader.uint64();
    	                    break;
    	                case 4:
    	                    message.label = reader.string();
    	                    break;
    	                case 5:
    	                    message.msg = reader.bytes();
    	                    break;
    	                case 6:
    	                    message.funds.push(coin_1.Coin.decode(reader, reader.uint32()));
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            sender: (0, helpers_1.isSet)(object.sender) ? String(object.sender) : "",
    	            admin: (0, helpers_1.isSet)(object.admin) ? String(object.admin) : "",
    	            codeId: (0, helpers_1.isSet)(object.codeId) ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO,
    	            label: (0, helpers_1.isSet)(object.label) ? String(object.label) : "",
    	            msg: (0, helpers_1.isSet)(object.msg) ? (0, helpers_1.bytesFromBase64)(object.msg) : new Uint8Array(),
    	            funds: Array.isArray(object?.funds) ? object.funds.map((e) => coin_1.Coin.fromJSON(e)) : [],
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.sender !== undefined && (obj.sender = message.sender);
    	        message.admin !== undefined && (obj.admin = message.admin);
    	        message.codeId !== undefined && (obj.codeId = (message.codeId || helpers_1.Long.UZERO).toString());
    	        message.label !== undefined && (obj.label = message.label);
    	        message.msg !== undefined &&
    	            (obj.msg = (0, helpers_1.base64FromBytes)(message.msg !== undefined ? message.msg : new Uint8Array()));
    	        if (message.funds) {
    	            obj.funds = message.funds.map((e) => (e ? coin_1.Coin.toJSON(e) : undefined));
    	        }
    	        else {
    	            obj.funds = [];
    	        }
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseMsgInstantiateContract();
    	        message.sender = object.sender ?? "";
    	        message.admin = object.admin ?? "";
    	        message.codeId =
    	            object.codeId !== undefined && object.codeId !== null ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO;
    	        message.label = object.label ?? "";
    	        message.msg = object.msg ?? new Uint8Array();
    	        message.funds = object.funds?.map((e) => coin_1.Coin.fromPartial(e)) || [];
    	        return message;
    	    },
    	};
    	function createBaseMsgInstantiateContract2() {
    	    return {
    	        sender: "",
    	        admin: "",
    	        codeId: helpers_1.Long.UZERO,
    	        label: "",
    	        msg: new Uint8Array(),
    	        funds: [],
    	        salt: new Uint8Array(),
    	        fixMsg: false,
    	    };
    	}
    	exports.MsgInstantiateContract2 = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.sender !== "") {
    	            writer.uint32(10).string(message.sender);
    	        }
    	        if (message.admin !== "") {
    	            writer.uint32(18).string(message.admin);
    	        }
    	        if (!message.codeId.isZero()) {
    	            writer.uint32(24).uint64(message.codeId);
    	        }
    	        if (message.label !== "") {
    	            writer.uint32(34).string(message.label);
    	        }
    	        if (message.msg.length !== 0) {
    	            writer.uint32(42).bytes(message.msg);
    	        }
    	        for (const v of message.funds) {
    	            coin_1.Coin.encode(v, writer.uint32(50).fork()).ldelim();
    	        }
    	        if (message.salt.length !== 0) {
    	            writer.uint32(58).bytes(message.salt);
    	        }
    	        if (message.fixMsg === true) {
    	            writer.uint32(64).bool(message.fixMsg);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgInstantiateContract2();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.sender = reader.string();
    	                    break;
    	                case 2:
    	                    message.admin = reader.string();
    	                    break;
    	                case 3:
    	                    message.codeId = reader.uint64();
    	                    break;
    	                case 4:
    	                    message.label = reader.string();
    	                    break;
    	                case 5:
    	                    message.msg = reader.bytes();
    	                    break;
    	                case 6:
    	                    message.funds.push(coin_1.Coin.decode(reader, reader.uint32()));
    	                    break;
    	                case 7:
    	                    message.salt = reader.bytes();
    	                    break;
    	                case 8:
    	                    message.fixMsg = reader.bool();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            sender: (0, helpers_1.isSet)(object.sender) ? String(object.sender) : "",
    	            admin: (0, helpers_1.isSet)(object.admin) ? String(object.admin) : "",
    	            codeId: (0, helpers_1.isSet)(object.codeId) ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO,
    	            label: (0, helpers_1.isSet)(object.label) ? String(object.label) : "",
    	            msg: (0, helpers_1.isSet)(object.msg) ? (0, helpers_1.bytesFromBase64)(object.msg) : new Uint8Array(),
    	            funds: Array.isArray(object?.funds) ? object.funds.map((e) => coin_1.Coin.fromJSON(e)) : [],
    	            salt: (0, helpers_1.isSet)(object.salt) ? (0, helpers_1.bytesFromBase64)(object.salt) : new Uint8Array(),
    	            fixMsg: (0, helpers_1.isSet)(object.fixMsg) ? Boolean(object.fixMsg) : false,
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.sender !== undefined && (obj.sender = message.sender);
    	        message.admin !== undefined && (obj.admin = message.admin);
    	        message.codeId !== undefined && (obj.codeId = (message.codeId || helpers_1.Long.UZERO).toString());
    	        message.label !== undefined && (obj.label = message.label);
    	        message.msg !== undefined &&
    	            (obj.msg = (0, helpers_1.base64FromBytes)(message.msg !== undefined ? message.msg : new Uint8Array()));
    	        if (message.funds) {
    	            obj.funds = message.funds.map((e) => (e ? coin_1.Coin.toJSON(e) : undefined));
    	        }
    	        else {
    	            obj.funds = [];
    	        }
    	        message.salt !== undefined &&
    	            (obj.salt = (0, helpers_1.base64FromBytes)(message.salt !== undefined ? message.salt : new Uint8Array()));
    	        message.fixMsg !== undefined && (obj.fixMsg = message.fixMsg);
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseMsgInstantiateContract2();
    	        message.sender = object.sender ?? "";
    	        message.admin = object.admin ?? "";
    	        message.codeId =
    	            object.codeId !== undefined && object.codeId !== null ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO;
    	        message.label = object.label ?? "";
    	        message.msg = object.msg ?? new Uint8Array();
    	        message.funds = object.funds?.map((e) => coin_1.Coin.fromPartial(e)) || [];
    	        message.salt = object.salt ?? new Uint8Array();
    	        message.fixMsg = object.fixMsg ?? false;
    	        return message;
    	    },
    	};
    	function createBaseMsgInstantiateContractResponse() {
    	    return {
    	        address: "",
    	        data: new Uint8Array(),
    	    };
    	}
    	exports.MsgInstantiateContractResponse = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.address !== "") {
    	            writer.uint32(10).string(message.address);
    	        }
    	        if (message.data.length !== 0) {
    	            writer.uint32(18).bytes(message.data);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgInstantiateContractResponse();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.address = reader.string();
    	                    break;
    	                case 2:
    	                    message.data = reader.bytes();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            address: (0, helpers_1.isSet)(object.address) ? String(object.address) : "",
    	            data: (0, helpers_1.isSet)(object.data) ? (0, helpers_1.bytesFromBase64)(object.data) : new Uint8Array(),
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.address !== undefined && (obj.address = message.address);
    	        message.data !== undefined &&
    	            (obj.data = (0, helpers_1.base64FromBytes)(message.data !== undefined ? message.data : new Uint8Array()));
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseMsgInstantiateContractResponse();
    	        message.address = object.address ?? "";
    	        message.data = object.data ?? new Uint8Array();
    	        return message;
    	    },
    	};
    	function createBaseMsgInstantiateContract2Response() {
    	    return {
    	        address: "",
    	        data: new Uint8Array(),
    	    };
    	}
    	exports.MsgInstantiateContract2Response = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.address !== "") {
    	            writer.uint32(10).string(message.address);
    	        }
    	        if (message.data.length !== 0) {
    	            writer.uint32(18).bytes(message.data);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgInstantiateContract2Response();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.address = reader.string();
    	                    break;
    	                case 2:
    	                    message.data = reader.bytes();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            address: (0, helpers_1.isSet)(object.address) ? String(object.address) : "",
    	            data: (0, helpers_1.isSet)(object.data) ? (0, helpers_1.bytesFromBase64)(object.data) : new Uint8Array(),
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.address !== undefined && (obj.address = message.address);
    	        message.data !== undefined &&
    	            (obj.data = (0, helpers_1.base64FromBytes)(message.data !== undefined ? message.data : new Uint8Array()));
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseMsgInstantiateContract2Response();
    	        message.address = object.address ?? "";
    	        message.data = object.data ?? new Uint8Array();
    	        return message;
    	    },
    	};
    	function createBaseMsgExecuteContract() {
    	    return {
    	        sender: "",
    	        contract: "",
    	        msg: new Uint8Array(),
    	        funds: [],
    	    };
    	}
    	exports.MsgExecuteContract = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.sender !== "") {
    	            writer.uint32(10).string(message.sender);
    	        }
    	        if (message.contract !== "") {
    	            writer.uint32(18).string(message.contract);
    	        }
    	        if (message.msg.length !== 0) {
    	            writer.uint32(26).bytes(message.msg);
    	        }
    	        for (const v of message.funds) {
    	            coin_1.Coin.encode(v, writer.uint32(42).fork()).ldelim();
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgExecuteContract();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.sender = reader.string();
    	                    break;
    	                case 2:
    	                    message.contract = reader.string();
    	                    break;
    	                case 3:
    	                    message.msg = reader.bytes();
    	                    break;
    	                case 5:
    	                    message.funds.push(coin_1.Coin.decode(reader, reader.uint32()));
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            sender: (0, helpers_1.isSet)(object.sender) ? String(object.sender) : "",
    	            contract: (0, helpers_1.isSet)(object.contract) ? String(object.contract) : "",
    	            msg: (0, helpers_1.isSet)(object.msg) ? (0, helpers_1.bytesFromBase64)(object.msg) : new Uint8Array(),
    	            funds: Array.isArray(object?.funds) ? object.funds.map((e) => coin_1.Coin.fromJSON(e)) : [],
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.sender !== undefined && (obj.sender = message.sender);
    	        message.contract !== undefined && (obj.contract = message.contract);
    	        message.msg !== undefined &&
    	            (obj.msg = (0, helpers_1.base64FromBytes)(message.msg !== undefined ? message.msg : new Uint8Array()));
    	        if (message.funds) {
    	            obj.funds = message.funds.map((e) => (e ? coin_1.Coin.toJSON(e) : undefined));
    	        }
    	        else {
    	            obj.funds = [];
    	        }
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseMsgExecuteContract();
    	        message.sender = object.sender ?? "";
    	        message.contract = object.contract ?? "";
    	        message.msg = object.msg ?? new Uint8Array();
    	        message.funds = object.funds?.map((e) => coin_1.Coin.fromPartial(e)) || [];
    	        return message;
    	    },
    	};
    	function createBaseMsgExecuteContractResponse() {
    	    return {
    	        data: new Uint8Array(),
    	    };
    	}
    	exports.MsgExecuteContractResponse = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.data.length !== 0) {
    	            writer.uint32(10).bytes(message.data);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgExecuteContractResponse();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.data = reader.bytes();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            data: (0, helpers_1.isSet)(object.data) ? (0, helpers_1.bytesFromBase64)(object.data) : new Uint8Array(),
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.data !== undefined &&
    	            (obj.data = (0, helpers_1.base64FromBytes)(message.data !== undefined ? message.data : new Uint8Array()));
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseMsgExecuteContractResponse();
    	        message.data = object.data ?? new Uint8Array();
    	        return message;
    	    },
    	};
    	function createBaseMsgMigrateContract() {
    	    return {
    	        sender: "",
    	        contract: "",
    	        codeId: helpers_1.Long.UZERO,
    	        msg: new Uint8Array(),
    	    };
    	}
    	exports.MsgMigrateContract = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.sender !== "") {
    	            writer.uint32(10).string(message.sender);
    	        }
    	        if (message.contract !== "") {
    	            writer.uint32(18).string(message.contract);
    	        }
    	        if (!message.codeId.isZero()) {
    	            writer.uint32(24).uint64(message.codeId);
    	        }
    	        if (message.msg.length !== 0) {
    	            writer.uint32(34).bytes(message.msg);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgMigrateContract();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.sender = reader.string();
    	                    break;
    	                case 2:
    	                    message.contract = reader.string();
    	                    break;
    	                case 3:
    	                    message.codeId = reader.uint64();
    	                    break;
    	                case 4:
    	                    message.msg = reader.bytes();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            sender: (0, helpers_1.isSet)(object.sender) ? String(object.sender) : "",
    	            contract: (0, helpers_1.isSet)(object.contract) ? String(object.contract) : "",
    	            codeId: (0, helpers_1.isSet)(object.codeId) ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO,
    	            msg: (0, helpers_1.isSet)(object.msg) ? (0, helpers_1.bytesFromBase64)(object.msg) : new Uint8Array(),
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.sender !== undefined && (obj.sender = message.sender);
    	        message.contract !== undefined && (obj.contract = message.contract);
    	        message.codeId !== undefined && (obj.codeId = (message.codeId || helpers_1.Long.UZERO).toString());
    	        message.msg !== undefined &&
    	            (obj.msg = (0, helpers_1.base64FromBytes)(message.msg !== undefined ? message.msg : new Uint8Array()));
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseMsgMigrateContract();
    	        message.sender = object.sender ?? "";
    	        message.contract = object.contract ?? "";
    	        message.codeId =
    	            object.codeId !== undefined && object.codeId !== null ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO;
    	        message.msg = object.msg ?? new Uint8Array();
    	        return message;
    	    },
    	};
    	function createBaseMsgMigrateContractResponse() {
    	    return {
    	        data: new Uint8Array(),
    	    };
    	}
    	exports.MsgMigrateContractResponse = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.data.length !== 0) {
    	            writer.uint32(10).bytes(message.data);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgMigrateContractResponse();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.data = reader.bytes();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            data: (0, helpers_1.isSet)(object.data) ? (0, helpers_1.bytesFromBase64)(object.data) : new Uint8Array(),
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.data !== undefined &&
    	            (obj.data = (0, helpers_1.base64FromBytes)(message.data !== undefined ? message.data : new Uint8Array()));
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseMsgMigrateContractResponse();
    	        message.data = object.data ?? new Uint8Array();
    	        return message;
    	    },
    	};
    	function createBaseMsgUpdateAdmin() {
    	    return {
    	        sender: "",
    	        newAdmin: "",
    	        contract: "",
    	    };
    	}
    	exports.MsgUpdateAdmin = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.sender !== "") {
    	            writer.uint32(10).string(message.sender);
    	        }
    	        if (message.newAdmin !== "") {
    	            writer.uint32(18).string(message.newAdmin);
    	        }
    	        if (message.contract !== "") {
    	            writer.uint32(26).string(message.contract);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgUpdateAdmin();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.sender = reader.string();
    	                    break;
    	                case 2:
    	                    message.newAdmin = reader.string();
    	                    break;
    	                case 3:
    	                    message.contract = reader.string();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            sender: (0, helpers_1.isSet)(object.sender) ? String(object.sender) : "",
    	            newAdmin: (0, helpers_1.isSet)(object.newAdmin) ? String(object.newAdmin) : "",
    	            contract: (0, helpers_1.isSet)(object.contract) ? String(object.contract) : "",
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.sender !== undefined && (obj.sender = message.sender);
    	        message.newAdmin !== undefined && (obj.newAdmin = message.newAdmin);
    	        message.contract !== undefined && (obj.contract = message.contract);
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseMsgUpdateAdmin();
    	        message.sender = object.sender ?? "";
    	        message.newAdmin = object.newAdmin ?? "";
    	        message.contract = object.contract ?? "";
    	        return message;
    	    },
    	};
    	function createBaseMsgUpdateAdminResponse() {
    	    return {};
    	}
    	exports.MsgUpdateAdminResponse = {
    	    encode(_, writer = _m0.Writer.create()) {
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgUpdateAdminResponse();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(_) {
    	        return {};
    	    },
    	    toJSON(_) {
    	        const obj = {};
    	        return obj;
    	    },
    	    fromPartial(_) {
    	        const message = createBaseMsgUpdateAdminResponse();
    	        return message;
    	    },
    	};
    	function createBaseMsgClearAdmin() {
    	    return {
    	        sender: "",
    	        contract: "",
    	    };
    	}
    	exports.MsgClearAdmin = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.sender !== "") {
    	            writer.uint32(10).string(message.sender);
    	        }
    	        if (message.contract !== "") {
    	            writer.uint32(26).string(message.contract);
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgClearAdmin();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.sender = reader.string();
    	                    break;
    	                case 3:
    	                    message.contract = reader.string();
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            sender: (0, helpers_1.isSet)(object.sender) ? String(object.sender) : "",
    	            contract: (0, helpers_1.isSet)(object.contract) ? String(object.contract) : "",
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.sender !== undefined && (obj.sender = message.sender);
    	        message.contract !== undefined && (obj.contract = message.contract);
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseMsgClearAdmin();
    	        message.sender = object.sender ?? "";
    	        message.contract = object.contract ?? "";
    	        return message;
    	    },
    	};
    	function createBaseMsgClearAdminResponse() {
    	    return {};
    	}
    	exports.MsgClearAdminResponse = {
    	    encode(_, writer = _m0.Writer.create()) {
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgClearAdminResponse();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(_) {
    	        return {};
    	    },
    	    toJSON(_) {
    	        const obj = {};
    	        return obj;
    	    },
    	    fromPartial(_) {
    	        const message = createBaseMsgClearAdminResponse();
    	        return message;
    	    },
    	};
    	function createBaseMsgUpdateInstantiateConfig() {
    	    return {
    	        sender: "",
    	        codeId: helpers_1.Long.UZERO,
    	        newInstantiatePermission: undefined,
    	    };
    	}
    	exports.MsgUpdateInstantiateConfig = {
    	    encode(message, writer = _m0.Writer.create()) {
    	        if (message.sender !== "") {
    	            writer.uint32(10).string(message.sender);
    	        }
    	        if (!message.codeId.isZero()) {
    	            writer.uint32(16).uint64(message.codeId);
    	        }
    	        if (message.newInstantiatePermission !== undefined) {
    	            types_1.AccessConfig.encode(message.newInstantiatePermission, writer.uint32(26).fork()).ldelim();
    	        }
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgUpdateInstantiateConfig();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                case 1:
    	                    message.sender = reader.string();
    	                    break;
    	                case 2:
    	                    message.codeId = reader.uint64();
    	                    break;
    	                case 3:
    	                    message.newInstantiatePermission = types_1.AccessConfig.decode(reader, reader.uint32());
    	                    break;
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(object) {
    	        return {
    	            sender: (0, helpers_1.isSet)(object.sender) ? String(object.sender) : "",
    	            codeId: (0, helpers_1.isSet)(object.codeId) ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO,
    	            newInstantiatePermission: (0, helpers_1.isSet)(object.newInstantiatePermission)
    	                ? types_1.AccessConfig.fromJSON(object.newInstantiatePermission)
    	                : undefined,
    	        };
    	    },
    	    toJSON(message) {
    	        const obj = {};
    	        message.sender !== undefined && (obj.sender = message.sender);
    	        message.codeId !== undefined && (obj.codeId = (message.codeId || helpers_1.Long.UZERO).toString());
    	        message.newInstantiatePermission !== undefined &&
    	            (obj.newInstantiatePermission = message.newInstantiatePermission
    	                ? types_1.AccessConfig.toJSON(message.newInstantiatePermission)
    	                : undefined);
    	        return obj;
    	    },
    	    fromPartial(object) {
    	        const message = createBaseMsgUpdateInstantiateConfig();
    	        message.sender = object.sender ?? "";
    	        message.codeId =
    	            object.codeId !== undefined && object.codeId !== null ? helpers_1.Long.fromValue(object.codeId) : helpers_1.Long.UZERO;
    	        message.newInstantiatePermission =
    	            object.newInstantiatePermission !== undefined && object.newInstantiatePermission !== null
    	                ? types_1.AccessConfig.fromPartial(object.newInstantiatePermission)
    	                : undefined;
    	        return message;
    	    },
    	};
    	function createBaseMsgUpdateInstantiateConfigResponse() {
    	    return {};
    	}
    	exports.MsgUpdateInstantiateConfigResponse = {
    	    encode(_, writer = _m0.Writer.create()) {
    	        return writer;
    	    },
    	    decode(input, length) {
    	        const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    	        let end = length === undefined ? reader.len : reader.pos + length;
    	        const message = createBaseMsgUpdateInstantiateConfigResponse();
    	        while (reader.pos < end) {
    	            const tag = reader.uint32();
    	            switch (tag >>> 3) {
    	                default:
    	                    reader.skipType(tag & 7);
    	                    break;
    	            }
    	        }
    	        return message;
    	    },
    	    fromJSON(_) {
    	        return {};
    	    },
    	    toJSON(_) {
    	        const obj = {};
    	        return obj;
    	    },
    	    fromPartial(_) {
    	        const message = createBaseMsgUpdateInstantiateConfigResponse();
    	        return message;
    	    },
    	};
    	class MsgClientImpl {
    	    constructor(rpc) {
    	        this.rpc = rpc;
    	        this.StoreCode = this.StoreCode.bind(this);
    	        this.InstantiateContract = this.InstantiateContract.bind(this);
    	        this.InstantiateContract2 = this.InstantiateContract2.bind(this);
    	        this.ExecuteContract = this.ExecuteContract.bind(this);
    	        this.MigrateContract = this.MigrateContract.bind(this);
    	        this.UpdateAdmin = this.UpdateAdmin.bind(this);
    	        this.ClearAdmin = this.ClearAdmin.bind(this);
    	        this.UpdateInstantiateConfig = this.UpdateInstantiateConfig.bind(this);
    	    }
    	    StoreCode(request) {
    	        const data = exports.MsgStoreCode.encode(request).finish();
    	        const promise = this.rpc.request("cosmwasm.wasm.v1.Msg", "StoreCode", data);
    	        return promise.then((data) => exports.MsgStoreCodeResponse.decode(new _m0.Reader(data)));
    	    }
    	    InstantiateContract(request) {
    	        const data = exports.MsgInstantiateContract.encode(request).finish();
    	        const promise = this.rpc.request("cosmwasm.wasm.v1.Msg", "InstantiateContract", data);
    	        return promise.then((data) => exports.MsgInstantiateContractResponse.decode(new _m0.Reader(data)));
    	    }
    	    InstantiateContract2(request) {
    	        const data = exports.MsgInstantiateContract2.encode(request).finish();
    	        const promise = this.rpc.request("cosmwasm.wasm.v1.Msg", "InstantiateContract2", data);
    	        return promise.then((data) => exports.MsgInstantiateContract2Response.decode(new _m0.Reader(data)));
    	    }
    	    ExecuteContract(request) {
    	        const data = exports.MsgExecuteContract.encode(request).finish();
    	        const promise = this.rpc.request("cosmwasm.wasm.v1.Msg", "ExecuteContract", data);
    	        return promise.then((data) => exports.MsgExecuteContractResponse.decode(new _m0.Reader(data)));
    	    }
    	    MigrateContract(request) {
    	        const data = exports.MsgMigrateContract.encode(request).finish();
    	        const promise = this.rpc.request("cosmwasm.wasm.v1.Msg", "MigrateContract", data);
    	        return promise.then((data) => exports.MsgMigrateContractResponse.decode(new _m0.Reader(data)));
    	    }
    	    UpdateAdmin(request) {
    	        const data = exports.MsgUpdateAdmin.encode(request).finish();
    	        const promise = this.rpc.request("cosmwasm.wasm.v1.Msg", "UpdateAdmin", data);
    	        return promise.then((data) => exports.MsgUpdateAdminResponse.decode(new _m0.Reader(data)));
    	    }
    	    ClearAdmin(request) {
    	        const data = exports.MsgClearAdmin.encode(request).finish();
    	        const promise = this.rpc.request("cosmwasm.wasm.v1.Msg", "ClearAdmin", data);
    	        return promise.then((data) => exports.MsgClearAdminResponse.decode(new _m0.Reader(data)));
    	    }
    	    UpdateInstantiateConfig(request) {
    	        const data = exports.MsgUpdateInstantiateConfig.encode(request).finish();
    	        const promise = this.rpc.request("cosmwasm.wasm.v1.Msg", "UpdateInstantiateConfig", data);
    	        return promise.then((data) => exports.MsgUpdateInstantiateConfigResponse.decode(new _m0.Reader(data)));
    	    }
    	}
    	exports.MsgClientImpl = MsgClientImpl;
    	
    } (tx));

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var AirdropMessageComposer = /** @class */ (function () {
        function AirdropMessageComposer(sender, contractAddress) {
            var _this = this;
            this.register = function (registerPayload, funds) {
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            register: registerPayload
                        })),
                        funds: funds
                    })
                };
            };
            this.fund = function (airdropId, funds) {
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            fund: airdropId
                        })),
                        funds: funds
                    })
                };
            };
            this.claim = function (claimPayload, funds) {
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            claim: claimPayload
                        })),
                        funds: funds
                    })
                };
            };
            this.close = function (airdropId, funds) {
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            close: airdropId
                        })),
                        funds: funds
                    })
                };
            };
            this.sender = sender;
            this.contractAddress = contractAddress;
            this.register = this.register.bind(this);
            this.fund = this.fund.bind(this);
            this.claim = this.claim.bind(this);
            this.close = this.close.bind(this);
        }
        return AirdropMessageComposer;
    }());

    var _2 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        AirdropMessageComposer: AirdropMessageComposer
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    function useAirdropListLabelsQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["airdropListLabels", client.contractAddress, JSON.stringify(args)], function () { return client.listLabels({
            limit: args.limit,
            order: args.order,
            startAfter: args.startAfter
        }); }, options);
    }
    function useAirdropGetLabelQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["airdropGetLabel", client.contractAddress, JSON.stringify(args)], function () { return client.getLabel({
            label: args.label
        }); }, options);
    }
    function useAirdropListClaimsQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["airdropListClaims", client.contractAddress, JSON.stringify(args)], function () { return client.listClaims({
            airdrop: args.airdrop,
            limit: args.limit,
            order: args.order,
            startAfter: args.startAfter
        }); }, options);
    }
    function useAirdropVerifyClaimQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["airdropVerifyClaim", client.contractAddress, JSON.stringify(args)], function () { return client.verifyClaim({
            claim: args.claim
        }); }, options);
    }
    function useAirdropGetClaimQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["airdropGetClaim", client.contractAddress, JSON.stringify(args)], function () { return client.getClaim({
            airdrop: args.airdrop,
            claimKey: args.claimKey
        }); }, options);
    }
    function useAirdropLatestAirdropIdQuery(_a) {
        var client = _a.client, options = _a.options;
        return reactQuery.useQuery(["airdropLatestAirdropId", client.contractAddress], function () { return client.latestAirdropId(); }, options);
    }
    function useAirdropListAirdropsQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["airdropListAirdrops", client.contractAddress, JSON.stringify(args)], function () { return client.listAirdrops({
            option: args.option
        }); }, options);
    }
    function useAirdropGetAirdropQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["airdropGetAirdrop", client.contractAddress, JSON.stringify(args)], function () { return client.getAirdrop({
            id: args.id
        }); }, options);
    }

    var _3 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        useAirdropListLabelsQuery: useAirdropListLabelsQuery,
        useAirdropGetLabelQuery: useAirdropGetLabelQuery,
        useAirdropListClaimsQuery: useAirdropListClaimsQuery,
        useAirdropVerifyClaimQuery: useAirdropVerifyClaimQuery,
        useAirdropGetClaimQuery: useAirdropGetClaimQuery,
        useAirdropLatestAirdropIdQuery: useAirdropLatestAirdropIdQuery,
        useAirdropListAirdropsQuery: useAirdropListAirdropsQuery,
        useAirdropGetAirdropQuery: useAirdropGetAirdropQuery
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */

    var _4 = /*#__PURE__*/Object.freeze({
        __proto__: null
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var CoreQueryClient = /** @class */ (function () {
        function CoreQueryClient(client, contractAddress) {
            var _this = this;
            this.getBalance = function (_a) {
                var account = _a.account;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_balance: {
                                    account: account
                                }
                            })];
                    });
                });
            };
            this.getTotalSupply = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            get_total_supply: {}
                        })];
                });
            }); };
            this.getConfig = function (_a) {
                var time = _a.time;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_config: {
                                    time: time
                                }
                            })];
                    });
                });
            };
            this.getFee = function (_a) {
                var time = _a.time;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_fee: {
                                    time: time
                                }
                            })];
                    });
                });
            };
            this.getPortfolio = function (_a) {
                var time = _a.time;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_portfolio: {
                                    time: time
                                }
                            })];
                    });
                });
            };
            this.getRebalance = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            get_rebalance: {}
                        })];
                });
            }); };
            this.getTradeInfo = function (_a) {
                var denomIn = _a.denomIn, denomOut = _a.denomOut;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_trade_info: {
                                    denom_in: denomIn,
                                    denom_out: denomOut
                                }
                            })];
                    });
                });
            };
            this.listTradeInfo = function (_a) {
                var denomIn = _a.denomIn, limit = _a.limit, order = _a.order, startAfter = _a.startAfter;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                list_trade_info: {
                                    denom_in: denomIn,
                                    limit: limit,
                                    order: order,
                                    start_after: startAfter
                                }
                            })];
                    });
                });
            };
            this.simulateMint = function (_a) {
                var amount = _a.amount, funds = _a.funds, time = _a.time;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                simulate_mint: {
                                    amount: amount,
                                    funds: funds,
                                    time: time
                                }
                            })];
                    });
                });
            };
            this.simulateBurn = function (_a) {
                var amount = _a.amount, time = _a.time;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                simulate_burn: {
                                    amount: amount,
                                    time: time
                                }
                            })];
                    });
                });
            };
            this.client = client;
            this.contractAddress = contractAddress;
            this.getBalance = this.getBalance.bind(this);
            this.getTotalSupply = this.getTotalSupply.bind(this);
            this.getConfig = this.getConfig.bind(this);
            this.getFee = this.getFee.bind(this);
            this.getPortfolio = this.getPortfolio.bind(this);
            this.getRebalance = this.getRebalance.bind(this);
            this.getTradeInfo = this.getTradeInfo.bind(this);
            this.listTradeInfo = this.listTradeInfo.bind(this);
            this.simulateMint = this.simulateMint.bind(this);
            this.simulateBurn = this.simulateBurn.bind(this);
        }
        return CoreQueryClient;
    }());
    var CoreClient = /** @class */ (function (_super) {
        __extends(CoreClient, _super);
        function CoreClient(client, sender, contractAddress) {
            var _this = _super.call(this, client, contractAddress) || this;
            _this.mint = function (_a, fee, memo, funds) {
                var amount = _a.amount, receiver = _a.receiver, refundTo = _a.refundTo;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    mint: {
                                        amount: amount,
                                        receiver: receiver,
                                        refund_to: refundTo
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.burn = function (_a, fee, memo, funds) {
                var redeemTo = _a.redeemTo;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    burn: {
                                        redeem_to: redeemTo
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.realize = function (fee, memo, funds) {
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    realize: {}
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _a.sent()];
                        }
                    });
                });
            };
            _this.gov = function (govMsg, fee, memo, funds) {
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    gov: govMsg
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _a.sent()];
                        }
                    });
                });
            };
            _this.rebalance = function (rebalanceMsg, fee, memo, funds) {
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    rebalance: rebalanceMsg
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _a.sent()];
                        }
                    });
                });
            };
            _this.client = client;
            _this.sender = sender;
            _this.contractAddress = contractAddress;
            _this.mint = _this.mint.bind(_this);
            _this.burn = _this.burn.bind(_this);
            _this.realize = _this.realize.bind(_this);
            _this.gov = _this.gov.bind(_this);
            _this.rebalance = _this.rebalance.bind(_this);
            return _this;
        }
        return CoreClient;
    }(CoreQueryClient));

    var _5 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        CoreQueryClient: CoreQueryClient,
        CoreClient: CoreClient
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var CoreMessageComposer = /** @class */ (function () {
        function CoreMessageComposer(sender, contractAddress) {
            var _this = this;
            this.mint = function (_a, funds) {
                var amount = _a.amount, receiver = _a.receiver, refundTo = _a.refundTo;
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            mint: {
                                amount: amount,
                                receiver: receiver,
                                refund_to: refundTo
                            }
                        })),
                        funds: funds
                    })
                };
            };
            this.burn = function (_a, funds) {
                var redeemTo = _a.redeemTo;
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            burn: {
                                redeem_to: redeemTo
                            }
                        })),
                        funds: funds
                    })
                };
            };
            this.realize = function (funds) {
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            realize: {}
                        })),
                        funds: funds
                    })
                };
            };
            this.gov = function (govMsg, funds) {
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            gov: govMsg
                        })),
                        funds: funds
                    })
                };
            };
            this.rebalance = function (rebalanceMsg, funds) {
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            rebalance: rebalanceMsg
                        })),
                        funds: funds
                    })
                };
            };
            this.sender = sender;
            this.contractAddress = contractAddress;
            this.mint = this.mint.bind(this);
            this.burn = this.burn.bind(this);
            this.realize = this.realize.bind(this);
            this.gov = this.gov.bind(this);
            this.rebalance = this.rebalance.bind(this);
        }
        return CoreMessageComposer;
    }());

    var _6 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        CoreMessageComposer: CoreMessageComposer
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    function useCoreSimulateBurnQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["coreSimulateBurn", client.contractAddress, JSON.stringify(args)], function () { return client.simulateBurn({
            amount: args.amount,
            time: args.time
        }); }, options);
    }
    function useCoreSimulateMintQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["coreSimulateMint", client.contractAddress, JSON.stringify(args)], function () { return client.simulateMint({
            amount: args.amount,
            funds: args.funds,
            time: args.time
        }); }, options);
    }
    function useCoreListTradeInfoQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["coreListTradeInfo", client.contractAddress, JSON.stringify(args)], function () { return client.listTradeInfo({
            denomIn: args.denomIn,
            limit: args.limit,
            order: args.order,
            startAfter: args.startAfter
        }); }, options);
    }
    function useCoreGetTradeInfoQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["coreGetTradeInfo", client.contractAddress, JSON.stringify(args)], function () { return client.getTradeInfo({
            denomIn: args.denomIn,
            denomOut: args.denomOut
        }); }, options);
    }
    function useCoreGetRebalanceQuery(_a) {
        var client = _a.client, options = _a.options;
        return reactQuery.useQuery(["coreGetRebalance", client.contractAddress], function () { return client.getRebalance(); }, options);
    }
    function useCoreGetPortfolioQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["coreGetPortfolio", client.contractAddress, JSON.stringify(args)], function () { return client.getPortfolio({
            time: args.time
        }); }, options);
    }
    function useCoreGetFeeQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["coreGetFee", client.contractAddress, JSON.stringify(args)], function () { return client.getFee({
            time: args.time
        }); }, options);
    }
    function useCoreGetConfigQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["coreGetConfig", client.contractAddress, JSON.stringify(args)], function () { return client.getConfig({
            time: args.time
        }); }, options);
    }
    function useCoreGetTotalSupplyQuery(_a) {
        var client = _a.client, options = _a.options;
        return reactQuery.useQuery(["coreGetTotalSupply", client.contractAddress], function () { return client.getTotalSupply(); }, options);
    }
    function useCoreGetBalanceQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["coreGetBalance", client.contractAddress, JSON.stringify(args)], function () { return client.getBalance({
            account: args.account
        }); }, options);
    }

    var _7 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        useCoreSimulateBurnQuery: useCoreSimulateBurnQuery,
        useCoreSimulateMintQuery: useCoreSimulateMintQuery,
        useCoreListTradeInfoQuery: useCoreListTradeInfoQuery,
        useCoreGetTradeInfoQuery: useCoreGetTradeInfoQuery,
        useCoreGetRebalanceQuery: useCoreGetRebalanceQuery,
        useCoreGetPortfolioQuery: useCoreGetPortfolioQuery,
        useCoreGetFeeQuery: useCoreGetFeeQuery,
        useCoreGetConfigQuery: useCoreGetConfigQuery,
        useCoreGetTotalSupplyQuery: useCoreGetTotalSupplyQuery,
        useCoreGetBalanceQuery: useCoreGetBalanceQuery
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */

    var _8 = /*#__PURE__*/Object.freeze({
        __proto__: null
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var PeripheryQueryClient = /** @class */ (function () {
        function PeripheryQueryClient(client, contractAddress) {
            var _this = this;
            this.simulateMintExactAmountOut = function (_a) {
                var coreAddr = _a.coreAddr, inputAsset = _a.inputAsset, outputAmount = _a.outputAmount, swapInfo = _a.swapInfo;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                simulate_mint_exact_amount_out: {
                                    core_addr: coreAddr,
                                    input_asset: inputAsset,
                                    output_amount: outputAmount,
                                    swap_info: swapInfo
                                }
                            })];
                    });
                });
            };
            this.simulateBurnExactAmountIn = function (_a) {
                var coreAddr = _a.coreAddr, inputAmount = _a.inputAmount, outputAsset = _a.outputAsset, swapInfo = _a.swapInfo;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                simulate_burn_exact_amount_in: {
                                    core_addr: coreAddr,
                                    input_amount: inputAmount,
                                    output_asset: outputAsset,
                                    swap_info: swapInfo
                                }
                            })];
                    });
                });
            };
            this.client = client;
            this.contractAddress = contractAddress;
            this.simulateMintExactAmountOut = this.simulateMintExactAmountOut.bind(this);
            this.simulateBurnExactAmountIn = this.simulateBurnExactAmountIn.bind(this);
        }
        return PeripheryQueryClient;
    }());
    var PeripheryClient = /** @class */ (function (_super) {
        __extends(PeripheryClient, _super);
        function PeripheryClient(client, sender, contractAddress) {
            var _this = _super.call(this, client, contractAddress) || this;
            _this.mintExactAmountOut = function (_a, fee, memo, funds) {
                var coreAddr = _a.coreAddr, inputAsset = _a.inputAsset, outputAmount = _a.outputAmount, swapInfo = _a.swapInfo;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    mint_exact_amount_out: {
                                        core_addr: coreAddr,
                                        input_asset: inputAsset,
                                        output_amount: outputAmount,
                                        swap_info: swapInfo
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.burnExactAmountIn = function (_a, fee, memo, funds) {
                var coreAddr = _a.coreAddr, minOutputAmount = _a.minOutputAmount, outputAsset = _a.outputAsset, swapInfo = _a.swapInfo;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    burn_exact_amount_in: {
                                        core_addr: coreAddr,
                                        min_output_amount: minOutputAmount,
                                        output_asset: outputAsset,
                                        swap_info: swapInfo
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.client = client;
            _this.sender = sender;
            _this.contractAddress = contractAddress;
            _this.mintExactAmountOut = _this.mintExactAmountOut.bind(_this);
            _this.burnExactAmountIn = _this.burnExactAmountIn.bind(_this);
            return _this;
        }
        return PeripheryClient;
    }(PeripheryQueryClient));

    var _9 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        PeripheryQueryClient: PeripheryQueryClient,
        PeripheryClient: PeripheryClient
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var PeripheryMessageComposer = /** @class */ (function () {
        function PeripheryMessageComposer(sender, contractAddress) {
            var _this = this;
            this.mintExactAmountOut = function (_a, funds) {
                var coreAddr = _a.coreAddr, inputAsset = _a.inputAsset, outputAmount = _a.outputAmount, swapInfo = _a.swapInfo;
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            mint_exact_amount_out: {
                                core_addr: coreAddr,
                                input_asset: inputAsset,
                                output_amount: outputAmount,
                                swap_info: swapInfo
                            }
                        })),
                        funds: funds
                    })
                };
            };
            this.burnExactAmountIn = function (_a, funds) {
                var coreAddr = _a.coreAddr, minOutputAmount = _a.minOutputAmount, outputAsset = _a.outputAsset, swapInfo = _a.swapInfo;
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            burn_exact_amount_in: {
                                core_addr: coreAddr,
                                min_output_amount: minOutputAmount,
                                output_asset: outputAsset,
                                swap_info: swapInfo
                            }
                        })),
                        funds: funds
                    })
                };
            };
            this.sender = sender;
            this.contractAddress = contractAddress;
            this.mintExactAmountOut = this.mintExactAmountOut.bind(this);
            this.burnExactAmountIn = this.burnExactAmountIn.bind(this);
        }
        return PeripheryMessageComposer;
    }());

    var _10 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        PeripheryMessageComposer: PeripheryMessageComposer
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    function usePeripherySimulateBurnExactAmountInQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["peripherySimulateBurnExactAmountIn", client.contractAddress, JSON.stringify(args)], function () { return client.simulateBurnExactAmountIn({
            coreAddr: args.coreAddr,
            inputAmount: args.inputAmount,
            outputAsset: args.outputAsset,
            swapInfo: args.swapInfo
        }); }, options);
    }
    function usePeripherySimulateMintExactAmountOutQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["peripherySimulateMintExactAmountOut", client.contractAddress, JSON.stringify(args)], function () { return client.simulateMintExactAmountOut({
            coreAddr: args.coreAddr,
            inputAsset: args.inputAsset,
            outputAmount: args.outputAmount,
            swapInfo: args.swapInfo
        }); }, options);
    }

    var _11 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        usePeripherySimulateBurnExactAmountInQuery: usePeripherySimulateBurnExactAmountInQuery,
        usePeripherySimulateMintExactAmountOutQuery: usePeripherySimulateMintExactAmountOutQuery
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var contracts$1;
    (function (contracts) {
        contracts.Airdrop = __assign(__assign(__assign(__assign({}, _0), _1), _2), _3);
        contracts.Core = __assign(__assign(__assign(__assign({}, _4), _5), _6), _7);
        contracts.Periphery = __assign(__assign(__assign(__assign({}, _8), _9), _10), _11);
    })(contracts$1 || (contracts$1 = {}));

    var contracts = contracts$1;

    exports["default"] = contracts;

    Object.defineProperty(exports, '__esModule', { value: true });

}));
//# sourceMappingURL=index.umd.js.map
