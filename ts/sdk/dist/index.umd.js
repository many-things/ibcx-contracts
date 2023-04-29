/*!
 * @many-things/ibcx-contracts-sdk v0.1.5
 * (c) frostornge <frostornge@gmail.com>
 * Released under the MIT OR Apache-2.0 License.
 */

(function (global, factory) {
    typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports, require('cosmjs-types/cosmwasm/wasm/v1/tx'), require('@cosmjs/encoding'), require('@tanstack/react-query')) :
    typeof define === 'function' && define.amd ? define(['exports', 'cosmjs-types/cosmwasm/wasm/v1/tx', '@cosmjs/encoding', '@tanstack/react-query'], factory) :
    (global = typeof globalThis !== 'undefined' ? globalThis : global || self, factory(global["counter-sdk"] = {}, global.tx, global.encoding, global.reactQuery));
})(this, (function (exports, tx, encoding, reactQuery) { 'use strict';

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
            this.getConfig = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            get_config: {}
                        })];
                });
            }); };
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
            this.getPauseInfo = function (_a) {
                var time = _a.time;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_pause_info: {
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
            this.getConfig = this.getConfig.bind(this);
            this.getFee = this.getFee.bind(this);
            this.getPauseInfo = this.getPauseInfo.bind(this);
            this.getPortfolio = this.getPortfolio.bind(this);
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
    function useCoreGetPortfolioQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["coreGetPortfolio", client.contractAddress, JSON.stringify(args)], function () { return client.getPortfolio({
            time: args.time
        }); }, options);
    }
    function useCoreGetPauseInfoQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["coreGetPauseInfo", client.contractAddress, JSON.stringify(args)], function () { return client.getPauseInfo({
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
        var client = _a.client, options = _a.options;
        return reactQuery.useQuery(["coreGetConfig", client.contractAddress], function () { return client.getConfig(); }, options);
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
        useCoreGetPortfolioQuery: useCoreGetPortfolioQuery,
        useCoreGetPauseInfoQuery: useCoreGetPauseInfoQuery,
        useCoreGetFeeQuery: useCoreGetFeeQuery,
        useCoreGetConfigQuery: useCoreGetConfigQuery,
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
    var FaucetQueryClient = /** @class */ (function () {
        function FaucetQueryClient(client, contractAddress) {
            var _this = this;
            this.listAliases = function (_a) {
                var limit = _a.limit, order = _a.order, startAfter = _a.startAfter;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                list_aliases: {
                                    limit: limit,
                                    order: order,
                                    start_after: startAfter
                                }
                            })];
                    });
                });
            };
            this.getToken = function (_a) {
                var denom = _a.denom;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_token: {
                                    denom: denom
                                }
                            })];
                    });
                });
            };
            this.listTokens = function (_a) {
                var limit = _a.limit, order = _a.order, startAfter = _a.startAfter;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                list_tokens: {
                                    limit: limit,
                                    order: order,
                                    start_after: startAfter
                                }
                            })];
                    });
                });
            };
            this.getLastTokenId = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            get_last_token_id: {}
                        })];
                });
            }); };
            this.getRole = function (_a) {
                var account = _a.account, denom = _a.denom;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_role: {
                                    account: account,
                                    denom: denom
                                }
                            })];
                    });
                });
            };
            this.listRoles = function (_a) {
                var denom = _a.denom, limit = _a.limit, order = _a.order, startAfter = _a.startAfter;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                list_roles: {
                                    denom: denom,
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
            this.listAliases = this.listAliases.bind(this);
            this.getToken = this.getToken.bind(this);
            this.listTokens = this.listTokens.bind(this);
            this.getLastTokenId = this.getLastTokenId.bind(this);
            this.getRole = this.getRole.bind(this);
            this.listRoles = this.listRoles.bind(this);
        }
        return FaucetQueryClient;
    }());
    var FaucetClient = /** @class */ (function (_super) {
        __extends(FaucetClient, _super);
        function FaucetClient(client, sender, contractAddress) {
            var _this = _super.call(this, client, contractAddress) || this;
            _this.create = function (_a, fee, memo, funds) {
                var config = _a.config, denom = _a.denom;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    create: {
                                        config: config,
                                        denom: denom
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.mint = function (_a, fee, memo, funds) {
                var amount = _a.amount, denom = _a.denom;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    mint: {
                                        amount: amount,
                                        denom: denom
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.burn = function (_a, fee, memo, funds) {
                var denom = _a.denom;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    burn: {
                                        denom: denom
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.grant = function (_a, fee, memo, funds) {
                var action = _a.action, denom = _a.denom, grantee = _a.grantee;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    grant: {
                                        action: action,
                                        denom: denom,
                                        grantee: grantee
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.revoke = function (_a, fee, memo, funds) {
                var action = _a.action, denom = _a.denom, revokee = _a.revokee;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    revoke: {
                                        action: action,
                                        denom: denom,
                                        revokee: revokee
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.release = function (_a, fee, memo, funds) {
                var action = _a.action, denom = _a.denom;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    release: {
                                        action: action,
                                        denom: denom
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.block = function (_a, fee, memo, funds) {
                var action = _a.action, denom = _a.denom;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    block: {
                                        action: action,
                                        denom: denom
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
            _this.create = _this.create.bind(_this);
            _this.mint = _this.mint.bind(_this);
            _this.burn = _this.burn.bind(_this);
            _this.grant = _this.grant.bind(_this);
            _this.revoke = _this.revoke.bind(_this);
            _this.release = _this.release.bind(_this);
            _this.block = _this.block.bind(_this);
            return _this;
        }
        return FaucetClient;
    }(FaucetQueryClient));

    var _9 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        FaucetQueryClient: FaucetQueryClient,
        FaucetClient: FaucetClient
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var FaucetMessageComposer = /** @class */ (function () {
        function FaucetMessageComposer(sender, contractAddress) {
            var _this = this;
            this.create = function (_a, funds) {
                var config = _a.config, denom = _a.denom;
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            create: {
                                config: config,
                                denom: denom
                            }
                        })),
                        funds: funds
                    })
                };
            };
            this.mint = function (_a, funds) {
                var amount = _a.amount, denom = _a.denom;
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            mint: {
                                amount: amount,
                                denom: denom
                            }
                        })),
                        funds: funds
                    })
                };
            };
            this.burn = function (_a, funds) {
                var denom = _a.denom;
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            burn: {
                                denom: denom
                            }
                        })),
                        funds: funds
                    })
                };
            };
            this.grant = function (_a, funds) {
                var action = _a.action, denom = _a.denom, grantee = _a.grantee;
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            grant: {
                                action: action,
                                denom: denom,
                                grantee: grantee
                            }
                        })),
                        funds: funds
                    })
                };
            };
            this.revoke = function (_a, funds) {
                var action = _a.action, denom = _a.denom, revokee = _a.revokee;
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            revoke: {
                                action: action,
                                denom: denom,
                                revokee: revokee
                            }
                        })),
                        funds: funds
                    })
                };
            };
            this.release = function (_a, funds) {
                var action = _a.action, denom = _a.denom;
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            release: {
                                action: action,
                                denom: denom
                            }
                        })),
                        funds: funds
                    })
                };
            };
            this.block = function (_a, funds) {
                var action = _a.action, denom = _a.denom;
                return {
                    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
                    value: tx.MsgExecuteContract.fromPartial({
                        sender: _this.sender,
                        contract: _this.contractAddress,
                        msg: encoding.toUtf8(JSON.stringify({
                            block: {
                                action: action,
                                denom: denom
                            }
                        })),
                        funds: funds
                    })
                };
            };
            this.sender = sender;
            this.contractAddress = contractAddress;
            this.create = this.create.bind(this);
            this.mint = this.mint.bind(this);
            this.burn = this.burn.bind(this);
            this.grant = this.grant.bind(this);
            this.revoke = this.revoke.bind(this);
            this.release = this.release.bind(this);
            this.block = this.block.bind(this);
        }
        return FaucetMessageComposer;
    }());

    var _10 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        FaucetMessageComposer: FaucetMessageComposer
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    function useFaucetListRolesQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["faucetListRoles", client.contractAddress, JSON.stringify(args)], function () { return client.listRoles({
            denom: args.denom,
            limit: args.limit,
            order: args.order,
            startAfter: args.startAfter
        }); }, options);
    }
    function useFaucetGetRoleQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["faucetGetRole", client.contractAddress, JSON.stringify(args)], function () { return client.getRole({
            account: args.account,
            denom: args.denom
        }); }, options);
    }
    function useFaucetGetLastTokenIdQuery(_a) {
        var client = _a.client, options = _a.options;
        return reactQuery.useQuery(["faucetGetLastTokenId", client.contractAddress], function () { return client.getLastTokenId(); }, options);
    }
    function useFaucetListTokensQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["faucetListTokens", client.contractAddress, JSON.stringify(args)], function () { return client.listTokens({
            limit: args.limit,
            order: args.order,
            startAfter: args.startAfter
        }); }, options);
    }
    function useFaucetGetTokenQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["faucetGetToken", client.contractAddress, JSON.stringify(args)], function () { return client.getToken({
            denom: args.denom
        }); }, options);
    }
    function useFaucetListAliasesQuery(_a) {
        var client = _a.client, args = _a.args, options = _a.options;
        return reactQuery.useQuery(["faucetListAliases", client.contractAddress, JSON.stringify(args)], function () { return client.listAliases({
            limit: args.limit,
            order: args.order,
            startAfter: args.startAfter
        }); }, options);
    }

    var _11 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        useFaucetListRolesQuery: useFaucetListRolesQuery,
        useFaucetGetRoleQuery: useFaucetGetRoleQuery,
        useFaucetGetLastTokenIdQuery: useFaucetGetLastTokenIdQuery,
        useFaucetListTokensQuery: useFaucetListTokensQuery,
        useFaucetGetTokenQuery: useFaucetGetTokenQuery,
        useFaucetListAliasesQuery: useFaucetListAliasesQuery
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */

    var _12 = /*#__PURE__*/Object.freeze({
        __proto__: null
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var PeripheryClient = /** @class */ (function () {
        function PeripheryClient(client, sender, contractAddress) {
            var _this = this;
            this.mintExactAmountOut = function (_a, fee, memo, funds) {
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
            this.burnExactAmountIn = function (_a, fee, memo, funds) {
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
            this.client = client;
            this.sender = sender;
            this.contractAddress = contractAddress;
            this.mintExactAmountOut = this.mintExactAmountOut.bind(this);
            this.burnExactAmountIn = this.burnExactAmountIn.bind(this);
        }
        return PeripheryClient;
    }());

    var _13 = /*#__PURE__*/Object.freeze({
        __proto__: null,
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

    var _14 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        PeripheryMessageComposer: PeripheryMessageComposer
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */

    var _15 = /*#__PURE__*/Object.freeze({
        __proto__: null
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
        contracts.Faucet = __assign(__assign(__assign(__assign({}, _8), _9), _10), _11);
        contracts.Periphery = __assign(__assign(__assign(__assign({}, _12), _13), _14), _15);
    })(contracts$1 || (contracts$1 = {}));

    var contracts = contracts$1;

    exports["default"] = contracts;

    Object.defineProperty(exports, '__esModule', { value: true });

}));
//# sourceMappingURL=index.umd.js.map
