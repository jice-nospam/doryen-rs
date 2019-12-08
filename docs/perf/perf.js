"use strict";

if( typeof Rust === "undefined" ) {
    var Rust = {};
}

(function( root, factory ) {
    if( typeof define === "function" && define.amd ) {
        define( [], factory );
    } else if( typeof module === "object" && module.exports ) {
        module.exports = factory();
    } else {
        Rust.perf = factory();
    }
}( this, function() {
    return (function( module_factory ) {
        var instance = module_factory();

        if( typeof process === "object" && typeof process.versions === "object" && typeof process.versions.node === "string" ) {
            var fs = require( "fs" );
            var path = require( "path" );
            var wasm_path = path.join( __dirname, "perf.wasm" );
            var buffer = fs.readFileSync( wasm_path );
            var mod = new WebAssembly.Module( buffer );
            var wasm_instance = new WebAssembly.Instance( mod, instance.imports );
            return instance.initialize( wasm_instance );
        } else {
            var file = fetch( "perf.wasm", {credentials: "same-origin"} );

            var wasm_instance = ( typeof WebAssembly.instantiateStreaming === "function"
                ? WebAssembly.instantiateStreaming( file, instance.imports )
                    .then( function( result ) { return result.instance; } )

                : file
                    .then( function( response ) { return response.arrayBuffer(); } )
                    .then( function( bytes ) { return WebAssembly.compile( bytes ); } )
                    .then( function( mod ) { return WebAssembly.instantiate( mod, instance.imports ) } ) );

            return wasm_instance
                .then( function( wasm_instance ) {
                    var exports = instance.initialize( wasm_instance );
                    console.log( "Finished loading Rust wasm module 'perf'" );
                    return exports;
                })
                .catch( function( error ) {
                    console.log( "Error loading Rust wasm module 'perf':", error );
                    throw error;
                });
        }
    }( function() {
    var Module = {};

    Module.STDWEB_PRIVATE = {};

// This is based on code from Emscripten's preamble.js.
Module.STDWEB_PRIVATE.to_utf8 = function to_utf8( str, addr ) {
    var HEAPU8 = Module.HEAPU8;
    for( var i = 0; i < str.length; ++i ) {
        // Gotcha: charCodeAt returns a 16-bit word that is a UTF-16 encoded code unit, not a Unicode code point of the character! So decode UTF16->UTF32->UTF8.
        // See http://unicode.org/faq/utf_bom.html#utf16-3
        // For UTF8 byte structure, see http://en.wikipedia.org/wiki/UTF-8#Description and https://www.ietf.org/rfc/rfc2279.txt and https://tools.ietf.org/html/rfc3629
        var u = str.charCodeAt( i ); // possibly a lead surrogate
        if( u >= 0xD800 && u <= 0xDFFF ) {
            u = 0x10000 + ((u & 0x3FF) << 10) | (str.charCodeAt( ++i ) & 0x3FF);
        }

        if( u <= 0x7F ) {
            HEAPU8[ addr++ ] = u;
        } else if( u <= 0x7FF ) {
            HEAPU8[ addr++ ] = 0xC0 | (u >> 6);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else if( u <= 0xFFFF ) {
            HEAPU8[ addr++ ] = 0xE0 | (u >> 12);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else if( u <= 0x1FFFFF ) {
            HEAPU8[ addr++ ] = 0xF0 | (u >> 18);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else if( u <= 0x3FFFFFF ) {
            HEAPU8[ addr++ ] = 0xF8 | (u >> 24);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 18) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else {
            HEAPU8[ addr++ ] = 0xFC | (u >> 30);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 24) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 18) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        }
    }
};

Module.STDWEB_PRIVATE.noop = function() {};
Module.STDWEB_PRIVATE.to_js = function to_js( address ) {
    var kind = Module.HEAPU8[ address + 12 ];
    if( kind === 0 ) {
        return undefined;
    } else if( kind === 1 ) {
        return null;
    } else if( kind === 2 ) {
        return Module.HEAP32[ address / 4 ];
    } else if( kind === 3 ) {
        return Module.HEAPF64[ address / 8 ];
    } else if( kind === 4 ) {
        var pointer = Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        return Module.STDWEB_PRIVATE.to_js_string( pointer, length );
    } else if( kind === 5 ) {
        return false;
    } else if( kind === 6 ) {
        return true;
    } else if( kind === 7 ) {
        var pointer = Module.STDWEB_PRIVATE.arena + Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        var output = [];
        for( var i = 0; i < length; ++i ) {
            output.push( Module.STDWEB_PRIVATE.to_js( pointer + i * 16 ) );
        }
        return output;
    } else if( kind === 8 ) {
        var arena = Module.STDWEB_PRIVATE.arena;
        var value_array_pointer = arena + Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        var key_array_pointer = arena + Module.HEAPU32[ (address + 8) / 4 ];
        var output = {};
        for( var i = 0; i < length; ++i ) {
            var key_pointer = Module.HEAPU32[ (key_array_pointer + i * 8) / 4 ];
            var key_length = Module.HEAPU32[ (key_array_pointer + 4 + i * 8) / 4 ];
            var key = Module.STDWEB_PRIVATE.to_js_string( key_pointer, key_length );
            var value = Module.STDWEB_PRIVATE.to_js( value_array_pointer + i * 16 );
            output[ key ] = value;
        }
        return output;
    } else if( kind === 9 ) {
        return Module.STDWEB_PRIVATE.acquire_js_reference( Module.HEAP32[ address / 4 ] );
    } else if( kind === 10 || kind === 12 || kind === 13 ) {
        var adapter_pointer = Module.HEAPU32[ address / 4 ];
        var pointer = Module.HEAPU32[ (address + 4) / 4 ];
        var deallocator_pointer = Module.HEAPU32[ (address + 8) / 4 ];
        var num_ongoing_calls = 0;
        var drop_queued = false;
        var output = function() {
            if( pointer === 0 || drop_queued === true ) {
                if (kind === 10) {
                    throw new ReferenceError( "Already dropped Rust function called!" );
                } else if (kind === 12) {
                    throw new ReferenceError( "Already dropped FnMut function called!" );
                } else {
                    throw new ReferenceError( "Already called or dropped FnOnce function called!" );
                }
            }

            var function_pointer = pointer;
            if (kind === 13) {
                output.drop = Module.STDWEB_PRIVATE.noop;
                pointer = 0;
            }

            if (num_ongoing_calls !== 0) {
                if (kind === 12 || kind === 13) {
                    throw new ReferenceError( "FnMut function called multiple times concurrently!" );
                }
            }

            var args = Module.STDWEB_PRIVATE.alloc( 16 );
            Module.STDWEB_PRIVATE.serialize_array( args, arguments );

            try {
                num_ongoing_calls += 1;
                Module.STDWEB_PRIVATE.dyncall( "vii", adapter_pointer, [function_pointer, args] );
                var result = Module.STDWEB_PRIVATE.tmp;
                Module.STDWEB_PRIVATE.tmp = null;
            } finally {
                num_ongoing_calls -= 1;
            }

            if( drop_queued === true && num_ongoing_calls === 0 ) {
                output.drop();
            }

            return result;
        };

        output.drop = function() {
            if (num_ongoing_calls !== 0) {
                drop_queued = true;
                return;
            }

            output.drop = Module.STDWEB_PRIVATE.noop;
            var function_pointer = pointer;
            pointer = 0;

            if (function_pointer != 0) {
                Module.STDWEB_PRIVATE.dyncall( "vi", deallocator_pointer, [function_pointer] );
            }
        };

        return output;
    } else if( kind === 14 ) {
        var pointer = Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        var array_kind = Module.HEAPU32[ (address + 8) / 4 ];
        var pointer_end = pointer + length;

        switch( array_kind ) {
            case 0:
                return Module.HEAPU8.subarray( pointer, pointer_end );
            case 1:
                return Module.HEAP8.subarray( pointer, pointer_end );
            case 2:
                return Module.HEAPU16.subarray( pointer, pointer_end );
            case 3:
                return Module.HEAP16.subarray( pointer, pointer_end );
            case 4:
                return Module.HEAPU32.subarray( pointer, pointer_end );
            case 5:
                return Module.HEAP32.subarray( pointer, pointer_end );
            case 6:
                return Module.HEAPF32.subarray( pointer, pointer_end );
            case 7:
                return Module.HEAPF64.subarray( pointer, pointer_end );
        }
    } else if( kind === 15 ) {
        return Module.STDWEB_PRIVATE.get_raw_value( Module.HEAPU32[ address / 4 ] );
    }
};

Module.STDWEB_PRIVATE.serialize_object = function serialize_object( address, value ) {
    var keys = Object.keys( value );
    var length = keys.length;
    var key_array_pointer = Module.STDWEB_PRIVATE.alloc( length * 8 );
    var value_array_pointer = Module.STDWEB_PRIVATE.alloc( length * 16 );
    Module.HEAPU8[ address + 12 ] = 8;
    Module.HEAPU32[ address / 4 ] = value_array_pointer;
    Module.HEAPU32[ (address + 4) / 4 ] = length;
    Module.HEAPU32[ (address + 8) / 4 ] = key_array_pointer;
    for( var i = 0; i < length; ++i ) {
        var key = keys[ i ];
        var key_address = key_array_pointer + i * 8;
        Module.STDWEB_PRIVATE.to_utf8_string( key_address, key );

        Module.STDWEB_PRIVATE.from_js( value_array_pointer + i * 16, value[ key ] );
    }
};

Module.STDWEB_PRIVATE.serialize_array = function serialize_array( address, value ) {
    var length = value.length;
    var pointer = Module.STDWEB_PRIVATE.alloc( length * 16 );
    Module.HEAPU8[ address + 12 ] = 7;
    Module.HEAPU32[ address / 4 ] = pointer;
    Module.HEAPU32[ (address + 4) / 4 ] = length;
    for( var i = 0; i < length; ++i ) {
        Module.STDWEB_PRIVATE.from_js( pointer + i * 16, value[ i ] );
    }
};

// New browsers and recent Node
var cachedEncoder = ( typeof TextEncoder === "function"
    ? new TextEncoder( "utf-8" )
    // Old Node (before v11)
    : ( typeof util === "object" && util && typeof util.TextEncoder === "function"
        ? new util.TextEncoder( "utf-8" )
        // Old browsers
        : null ) );

if ( cachedEncoder != null ) {
    Module.STDWEB_PRIVATE.to_utf8_string = function to_utf8_string( address, value ) {
        var buffer = cachedEncoder.encode( value );
        var length = buffer.length;
        var pointer = 0;

        if ( length > 0 ) {
            pointer = Module.STDWEB_PRIVATE.alloc( length );
            Module.HEAPU8.set( buffer, pointer );
        }

        Module.HEAPU32[ address / 4 ] = pointer;
        Module.HEAPU32[ (address + 4) / 4 ] = length;
    };

} else {
    Module.STDWEB_PRIVATE.to_utf8_string = function to_utf8_string( address, value ) {
        var length = Module.STDWEB_PRIVATE.utf8_len( value );
        var pointer = 0;

        if ( length > 0 ) {
            pointer = Module.STDWEB_PRIVATE.alloc( length );
            Module.STDWEB_PRIVATE.to_utf8( value, pointer );
        }

        Module.HEAPU32[ address / 4 ] = pointer;
        Module.HEAPU32[ (address + 4) / 4 ] = length;
    };
}

Module.STDWEB_PRIVATE.from_js = function from_js( address, value ) {
    var kind = Object.prototype.toString.call( value );
    if( kind === "[object String]" ) {
        Module.HEAPU8[ address + 12 ] = 4;
        Module.STDWEB_PRIVATE.to_utf8_string( address, value );
    } else if( kind === "[object Number]" ) {
        if( value === (value|0) ) {
            Module.HEAPU8[ address + 12 ] = 2;
            Module.HEAP32[ address / 4 ] = value;
        } else {
            Module.HEAPU8[ address + 12 ] = 3;
            Module.HEAPF64[ address / 8 ] = value;
        }
    } else if( value === null ) {
        Module.HEAPU8[ address + 12 ] = 1;
    } else if( value === undefined ) {
        Module.HEAPU8[ address + 12 ] = 0;
    } else if( value === false ) {
        Module.HEAPU8[ address + 12 ] = 5;
    } else if( value === true ) {
        Module.HEAPU8[ address + 12 ] = 6;
    } else if( kind === "[object Symbol]" ) {
        var id = Module.STDWEB_PRIVATE.register_raw_value( value );
        Module.HEAPU8[ address + 12 ] = 15;
        Module.HEAP32[ address / 4 ] = id;
    } else {
        var refid = Module.STDWEB_PRIVATE.acquire_rust_reference( value );
        Module.HEAPU8[ address + 12 ] = 9;
        Module.HEAP32[ address / 4 ] = refid;
    }
};

// New browsers and recent Node
var cachedDecoder = ( typeof TextDecoder === "function"
    ? new TextDecoder( "utf-8" )
    // Old Node (before v11)
    : ( typeof util === "object" && util && typeof util.TextDecoder === "function"
        ? new util.TextDecoder( "utf-8" )
        // Old browsers
        : null ) );

if ( cachedDecoder != null ) {
    Module.STDWEB_PRIVATE.to_js_string = function to_js_string( index, length ) {
        return cachedDecoder.decode( Module.HEAPU8.subarray( index, index + length ) );
    };

} else {
    // This is ported from Rust's stdlib; it's faster than
    // the string conversion from Emscripten.
    Module.STDWEB_PRIVATE.to_js_string = function to_js_string( index, length ) {
        var HEAPU8 = Module.HEAPU8;
        index = index|0;
        length = length|0;
        var end = (index|0) + (length|0);
        var output = "";
        while( index < end ) {
            var x = HEAPU8[ index++ ];
            if( x < 128 ) {
                output += String.fromCharCode( x );
                continue;
            }
            var init = (x & (0x7F >> 2));
            var y = 0;
            if( index < end ) {
                y = HEAPU8[ index++ ];
            }
            var ch = (init << 6) | (y & 63);
            if( x >= 0xE0 ) {
                var z = 0;
                if( index < end ) {
                    z = HEAPU8[ index++ ];
                }
                var y_z = ((y & 63) << 6) | (z & 63);
                ch = init << 12 | y_z;
                if( x >= 0xF0 ) {
                    var w = 0;
                    if( index < end ) {
                        w = HEAPU8[ index++ ];
                    }
                    ch = (init & 7) << 18 | ((y_z << 6) | (w & 63));

                    output += String.fromCharCode( 0xD7C0 + (ch >> 10) );
                    ch = 0xDC00 + (ch & 0x3FF);
                }
            }
            output += String.fromCharCode( ch );
            continue;
        }
        return output;
    };
}

Module.STDWEB_PRIVATE.id_to_ref_map = {};
Module.STDWEB_PRIVATE.id_to_refcount_map = {};
Module.STDWEB_PRIVATE.ref_to_id_map = new WeakMap();
// Not all types can be stored in a WeakMap
Module.STDWEB_PRIVATE.ref_to_id_map_fallback = new Map();
Module.STDWEB_PRIVATE.last_refid = 1;

Module.STDWEB_PRIVATE.id_to_raw_value_map = {};
Module.STDWEB_PRIVATE.last_raw_value_id = 1;

Module.STDWEB_PRIVATE.acquire_rust_reference = function( reference ) {
    if( reference === undefined || reference === null ) {
        return 0;
    }

    var id_to_refcount_map = Module.STDWEB_PRIVATE.id_to_refcount_map;
    var id_to_ref_map = Module.STDWEB_PRIVATE.id_to_ref_map;
    var ref_to_id_map = Module.STDWEB_PRIVATE.ref_to_id_map;
    var ref_to_id_map_fallback = Module.STDWEB_PRIVATE.ref_to_id_map_fallback;

    var refid = ref_to_id_map.get( reference );
    if( refid === undefined ) {
        refid = ref_to_id_map_fallback.get( reference );
    }
    if( refid === undefined ) {
        refid = Module.STDWEB_PRIVATE.last_refid++;
        try {
            ref_to_id_map.set( reference, refid );
        } catch (e) {
            ref_to_id_map_fallback.set( reference, refid );
        }
    }

    if( refid in id_to_ref_map ) {
        id_to_refcount_map[ refid ]++;
    } else {
        id_to_ref_map[ refid ] = reference;
        id_to_refcount_map[ refid ] = 1;
    }

    return refid;
};

Module.STDWEB_PRIVATE.acquire_js_reference = function( refid ) {
    return Module.STDWEB_PRIVATE.id_to_ref_map[ refid ];
};

Module.STDWEB_PRIVATE.increment_refcount = function( refid ) {
    Module.STDWEB_PRIVATE.id_to_refcount_map[ refid ]++;
};

Module.STDWEB_PRIVATE.decrement_refcount = function( refid ) {
    var id_to_refcount_map = Module.STDWEB_PRIVATE.id_to_refcount_map;
    if( 0 == --id_to_refcount_map[ refid ] ) {
        var id_to_ref_map = Module.STDWEB_PRIVATE.id_to_ref_map;
        var ref_to_id_map_fallback = Module.STDWEB_PRIVATE.ref_to_id_map_fallback;
        var reference = id_to_ref_map[ refid ];
        delete id_to_ref_map[ refid ];
        delete id_to_refcount_map[ refid ];
        ref_to_id_map_fallback.delete(reference);
    }
};

Module.STDWEB_PRIVATE.register_raw_value = function( value ) {
    var id = Module.STDWEB_PRIVATE.last_raw_value_id++;
    Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ] = value;
    return id;
};

Module.STDWEB_PRIVATE.unregister_raw_value = function( id ) {
    delete Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ];
};

Module.STDWEB_PRIVATE.get_raw_value = function( id ) {
    return Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ];
};

Module.STDWEB_PRIVATE.alloc = function alloc( size ) {
    return Module.web_malloc( size );
};

Module.STDWEB_PRIVATE.dyncall = function( signature, ptr, args ) {
    return Module.web_table.get( ptr ).apply( null, args );
};

// This is based on code from Emscripten's preamble.js.
Module.STDWEB_PRIVATE.utf8_len = function utf8_len( str ) {
    var len = 0;
    for( var i = 0; i < str.length; ++i ) {
        // Gotcha: charCodeAt returns a 16-bit word that is a UTF-16 encoded code unit, not a Unicode code point of the character! So decode UTF16->UTF32->UTF8.
        // See http://unicode.org/faq/utf_bom.html#utf16-3
        var u = str.charCodeAt( i ); // possibly a lead surrogate
        if( u >= 0xD800 && u <= 0xDFFF ) {
            u = 0x10000 + ((u & 0x3FF) << 10) | (str.charCodeAt( ++i ) & 0x3FF);
        }

        if( u <= 0x7F ) {
            ++len;
        } else if( u <= 0x7FF ) {
            len += 2;
        } else if( u <= 0xFFFF ) {
            len += 3;
        } else if( u <= 0x1FFFFF ) {
            len += 4;
        } else if( u <= 0x3FFFFFF ) {
            len += 5;
        } else {
            len += 6;
        }
    }
    return len;
};

Module.STDWEB_PRIVATE.prepare_any_arg = function( value ) {
    var arg = Module.STDWEB_PRIVATE.alloc( 16 );
    Module.STDWEB_PRIVATE.from_js( arg, value );
    return arg;
};

Module.STDWEB_PRIVATE.acquire_tmp = function( dummy ) {
    var value = Module.STDWEB_PRIVATE.tmp;
    Module.STDWEB_PRIVATE.tmp = null;
    return value;
};



    var HEAP8 = null;
    var HEAP16 = null;
    var HEAP32 = null;
    var HEAPU8 = null;
    var HEAPU16 = null;
    var HEAPU32 = null;
    var HEAPF32 = null;
    var HEAPF64 = null;

    Object.defineProperty( Module, 'exports', { value: {} } );

    function __web_on_grow() {
        var buffer = Module.instance.exports.memory.buffer;
        HEAP8 = new Int8Array( buffer );
        HEAP16 = new Int16Array( buffer );
        HEAP32 = new Int32Array( buffer );
        HEAPU8 = new Uint8Array( buffer );
        HEAPU16 = new Uint16Array( buffer );
        HEAPU32 = new Uint32Array( buffer );
        HEAPF32 = new Float32Array( buffer );
        HEAPF64 = new Float64Array( buffer );
        Module.HEAP8 = HEAP8;
        Module.HEAP16 = HEAP16;
        Module.HEAP32 = HEAP32;
        Module.HEAPU8 = HEAPU8;
        Module.HEAPU16 = HEAPU16;
        Module.HEAPU32 = HEAPU32;
        Module.HEAPF32 = HEAPF32;
        Module.HEAPF64 = HEAPF64;
    }

    return {
        imports: {
            env: {
                "__cargo_web_snippet_00605188dd87406ccd80e97211f0336e62517924": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);var ctx=Module.gl.get(($0));var h=Module.gl.get(($1));ctx.useProgram(h.prog)
            },
            "__cargo_web_snippet_0316c735d05942851a64b39ce6afef9c3936dc89": function($0, $1, $2, $3) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){var ctx=Module.gl.get(($1));var h=Module.gl.get(($2));var r=ctx.getAttribLocation(h.prog,($3));return r>=0?r:null;})());
            },
            "__cargo_web_snippet_0326c402a09df06eb668ad14bff6431717298466": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);console.log(($0))
            },
            "__cargo_web_snippet_04ab7245f7f98f556cf91d4c0dd21db017e9d4f5": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).getBoundingClientRect().top;})());
            },
            "__cargo_web_snippet_06f31fdb8e557ba44aaaa2b7a5a6f57d459c4653": function($0, $1) {
                return Module.STDWEB_PRIVATE.acquire_rust_reference( Module.HEAPU8.slice( $0, $1 ) );
            },
            "__cargo_web_snippet_0a2c5e2af602f8a7a9ebbd8de6784b2b9ade93e8": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).ctrlKey;})());
            },
            "__cargo_web_snippet_0e54fd9c163fcf648ce0a395fde4500fd167a40b": function($0) {
                var r = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (r instanceof DOMException) && (r.name === "InvalidCharacterError");
            },
            "__cargo_web_snippet_0f503de1d61309643e0e13a7871406891e3691c9": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){return window;})());
            },
            "__cargo_web_snippet_213e295793ef2f18e73787d93e4f3fc909935143": function($0, $1, $2, $3, $4) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);var p=($0).concat(($1));var ctx=Module.gl.get(($2));var internal_fmt=($3);var fmt=internal_fmt;if(($4)){internal_fmt=ctx.DEPTH_COMPONENT16;}ctx.texImage2D(p[0],p[1],internal_fmt,p[3],p[4],0,fmt,p[6],null);
            },
            "__cargo_web_snippet_23639371cb88eaf0e4e3ff14ba63d1e5b5cea0b2": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).key;})());
            },
            "__cargo_web_snippet_24136cafbe01e239d28761cd37925bd458c4988f": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);var ctx=Module.gl.get(($0));var buf=Module.gl.get(($1));ctx.bindBuffer(($2),buf)
            },
            "__cargo_web_snippet_2566a94332f60241405fa45e6a379e8daa71af81": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof HTMLCanvasElement);
            },
            "__cargo_web_snippet_25ff8e5698651c3acff68ed138bf48b0a71719a6": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);var ctx=Module.gl.get(($0));var tex=Module.gl.get(($1));ctx.bindTexture(($2),tex)
            },
            "__cargo_web_snippet_29ef3a8aaefcd4cb1dfc6284d7e36127515d26ae": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof KeyboardEvent && o.type === "keydown");
            },
            "__cargo_web_snippet_2b5185132e2d66825672ac97dbe07d51c21e3856": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){return performance.now()/1000.0;})());
            },
            "__cargo_web_snippet_2e92e77c21dea6f3b01f9ba7ab22b0a1dae4a9ab": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).button;})());
            },
            "__cargo_web_snippet_33f5f6641245468022d789af32ef6937c59223c9": function($0, $1, $2, $3, $4, $5, $6) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);$5 = Module.STDWEB_PRIVATE.to_js($5);$6 = Module.STDWEB_PRIVATE.to_js($6);var ctx=Module.gl.get(($0));ctx.vertexAttribPointer(($1),($2),($3),($4),($5),($6));
            },
            "__cargo_web_snippet_3a52736f91788f64957d9353d5fe1dc56d6c0b3c": function($0, $1, $2, $3) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);var ctx=Module.gl.get(($0));ctx.bufferData(($1),($2),($3))
            },
            "__cargo_web_snippet_3c5e83d16a83fc7147ec91e2506438012952f55a": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof Element);
            },
            "__cargo_web_snippet_3c63e773071319eff09af93ceb0203933dc4f233": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof KeyboardEvent && o.type === "keyup");
            },
            "__cargo_web_snippet_3d3bcb1bb15693388a61d29f7ba643701fb617f3": function($0, $1, $2, $3) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);var p=[($0),($1)];var ctx=Module.gl.get(($2));var loc=Module.gl.get(($3));ctx.uniform2f(loc,p[0],p[1])
            },
            "__cargo_web_snippet_45571de900b0c8f647a8073289adcbdde6025d7d": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);var ctx=Module.gl.get(($0));var shader=Module.gl.get(($1));ctx.compileShader(shader);var compiled=ctx.getShaderParameter(shader,0x8B81);if(! compiled){console.log("ERROR in shader compilation:");console.log(ctx.getShaderInfoLog(shader));}
            },
            "__cargo_web_snippet_458ddc308a2045b4c4556b2b85e5294c98d29826": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);var ctx=Module.gl.get(($0));ctx.clear(($1))
            },
            "__cargo_web_snippet_45b00f6f53d96e0e624642a0afc8a2c5702f3685": function($0, $1, $2, $3, $4) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);Module.STDWEB_PRIVATE.from_js($0, (function(){var ctx=Module.gl.get(($1));return ctx.texParameteri(($2),($3),($4))})());
            },
            "__cargo_web_snippet_4e06a3790bf7c5d71cb31866f13dedbc663b75c5": function($0, $1, $2, $3) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){return[($1),($2),($3)]})());
            },
            "__cargo_web_snippet_500542edb621019a5dcfa85a6ec3dff648fab4d4": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);var ctx=Module.gl.get(($0));var p=($1);ctx.viewport(p[0],p[1],p[2],p[3]);
            },
            "__cargo_web_snippet_520e14e091ad5d654d8bcf292e8233a0174bd5ec": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);var ctx=Module.gl.get(($0));ctx.enable(($1));
            },
            "__cargo_web_snippet_5378f5cb8d70ac4792928a36e5faf957c0600f7f": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);var ctx=Module.gl.get(($0));ctx.enableVertexAttribArray(($1))
            },
            "__cargo_web_snippet_54c301f42021c68eb1c25310e36baf3b3b910e96": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);var array=($0);var pointer=($1);Module.HEAPU8.set(array,pointer);
            },
            "__cargo_web_snippet_614a3dd2adb7e9eac4a0ec6e59d37f87e0521c3b": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).error;})());
            },
            "__cargo_web_snippet_66992dd80dc3d58e6002425a9d78d42ea509a3a1": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){return window.devicePixelRatio;})());
            },
            "__cargo_web_snippet_6c1f25bf7c9104accb489618515bd1869f4ca315": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).clientX;})());
            },
            "__cargo_web_snippet_6fcce0aae651e2d748e085ff1f800f87625ff8c8": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){return document;})());
            },
            "__cargo_web_snippet_72f34c3354639612c5e8ae85301657f2088d54b2": function($0, $1, $2, $3, $4) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);var p=[($0),($1),($2),($3)];var ctx=Module.gl.get(($4));ctx.clearColor(p[0],p[1],p[2],p[3]);
            },
            "__cargo_web_snippet_72fc447820458c720c68d0d8e078ede631edd723": function($0, $1, $2) {
                console.error( 'Panic location:', Module.STDWEB_PRIVATE.to_js_string( $0, $1 ) + ':' + $2 );
            },
            "__cargo_web_snippet_74826961daf3d2c4eaad2d3f0cc83e4fcc5c8de8": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof MouseEvent && o.type === "mousemove");
            },
            "__cargo_web_snippet_7bead6b563d52eee65504adb6b76c5cacb5428d3": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);($0).preventDefault();
            },
            "__cargo_web_snippet_7c8dfab835dc8a552cd9d67f27d26624590e052c": function($0) {
                var r = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (r instanceof DOMException) && (r.name === "SyntaxError");
            },
            "__cargo_web_snippet_80d4c2448cb2af18023596a583004e4458bf110f": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);($0).style.cursor="none";
            },
            "__cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0": function($0) {
                Module.STDWEB_PRIVATE.decrement_refcount( $0 );
            },
            "__cargo_web_snippet_86d3581f577bd16bb76dfcf1320fa125038a9602": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).offsetWidth;})());
            },
            "__cargo_web_snippet_8b7cc1a90ecd19ea6290d7d0a840fd125dd3febb": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof Event && o.type === "resize");
            },
            "__cargo_web_snippet_90f020fc20631c35b56f9fc9cbb9c287a99d55be": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof MouseEvent && o.type === "mousedown");
            },
            "__cargo_web_snippet_91749aeb589cd0f9b17cbc01b2872ba709817982": function($0, $1, $2) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);Module.STDWEB_PRIVATE.from_js($0, (function(){try{return{value:function(){return($1).createElement(($2));}(),success:true};}catch(error){return{error:error,success:false};}})());
            },
            "__cargo_web_snippet_947e3c71a436d2534560c3daba2b3a52e02ec6d0": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).clientY;})());
            },
            "__cargo_web_snippet_97495987af1720d8a9a923fa4683a7b683e3acd6": function($0, $1) {
                console.error( 'Panic error message:', Module.STDWEB_PRIVATE.to_js_string( $0, $1 ) );
            },
            "__cargo_web_snippet_99c4eefdc8d4cc724135163b8c8665a1f3de99e4": function($0, $1, $2, $3) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){var listener=($1);($2).addEventListener(($3),listener);return listener;})());
            },
            "__cargo_web_snippet_9ca331f65fc765840c483d1701e6c7a4585d3d0f": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);{var ctx=Module.gl.get(($0));if(ctx.bindVertexArray){var vao=Module.gl.get(($1));ctx.bindVertexArray(vao);}}
            },
            "__cargo_web_snippet_9f22d4ca7bc938409787341b7db181f8dd41e6df": function($0) {
                Module.STDWEB_PRIVATE.increment_refcount( $0 );
            },
            "__cargo_web_snippet_9faad6b44e269256ee6fdc26c24f5c43f57a4bf8": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);var ctx=Module.gl.get(($0));ctx.blendFunc(($1),($2))
            },
            "__cargo_web_snippet_a152e8d0e8fac5476f30c1d19e4ab217dbcba73d": function($0, $1, $2) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);Module.STDWEB_PRIVATE.from_js($0, (function(){try{return{value:function(){return($1).querySelector(($2));}(),success:true};}catch(error){return{error:error,success:false};}})());
            },
            "__cargo_web_snippet_a1b895ff4f8af46c5eb82195e43b361298aacbac": function($0, $1, $2, $3) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){var ctx=Module.gl.get(($1));var h=Module.gl.get(($2));var name=($3);var uniform=h.uniform_names[name];if(name in h.uniform_names)return h.uniform_names[name];uniform=Module.gl.add(ctx.getUniformLocation(h.prog,name));h.uniform_names[name]=uniform;return uniform;})());
            },
            "__cargo_web_snippet_a21653520f5346fefeef4bdede86e03ba6d63e4b": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){var ctx=Module.gl.get(($1));if(ctx.createVertexArray){return Module.gl.add(ctx.createVertexArray());}else{return 0;}})());
            },
            "__cargo_web_snippet_a3340aa907e680a69e829ada20c93b8c00337efe": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).offsetHeight;})());
            },
            "__cargo_web_snippet_ab05f53189dacccf2d365ad26daa407d4f7abea9": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).value;})());
            },
            "__cargo_web_snippet_ad4e067decc06a4c91987e9f7e2452019870411e": function($0, $1, $2) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);Module.STDWEB_PRIVATE.from_js($0, (function(){var ctx=Module.gl.get(($1));return Module.gl.add(ctx.createShader(($2)));})());
            },
            "__cargo_web_snippet_aefaa9d37f5ee323edcd23f2b91f3aecdac17e52": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);($0).focus();
            },
            "__cargo_web_snippet_b06dde4acf09433b5190a4b001259fe5d4abcbc2": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).success;})());
            },
            "__cargo_web_snippet_b33a39de4ca954888e26fe9caa277138e808eeba": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).length;})());
            },
            "__cargo_web_snippet_b8ed88b85d1c7d2770711db9e9ceb7cb289a8c05": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);var ctx=Module.gl.get(($0));ctx.activeTexture(ctx.TEXTURE0+($1))
            },
            "__cargo_web_snippet_bddb2deeb28c1a52f02588464f2ecc52588dfb50": function($0, $1, $2, $3) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);var p=($0).concat(($1));var ctx=Module.gl.get(($2));ctx.texImage2D(p[0],p[1],p[2],p[3],p[4],0,p[2],p[6],($3));
            },
            "__cargo_web_snippet_c23b690c40e12d5801bb368f23cf8a0b9e7a6323": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);var ctx=Module.gl.get(($0));var shader=Module.gl.get(($1));ctx.shaderSource(shader,($2))
            },
            "__cargo_web_snippet_c38669646670bde0cacd1c090c1cb8d5a6a7d680": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).altKey;})());
            },
            "__cargo_web_snippet_c5218e1071166d54098e47c49dd44408541e469d": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);var ctx=Module.gl.get(($0));var loc=Module.gl.get(($1));ctx.uniform1f(loc,($2));
            },
            "__cargo_web_snippet_c5ad6f9b31d30f6b82003becad519241eba3af65": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){var ctx=Module.gl.get(($1));var h={};h.prog=ctx.createProgram();h.uniform_names={};return Module.gl.add(h);})());
            },
            "__cargo_web_snippet_c9e4f365c41e64cc323d6faaa12fc279d90766b1": function($0, $1, $2, $3, $4) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);Module.STDWEB_PRIVATE.from_js($0, (function(){return[($1),($2),($3),($4)]})());
            },
            "__cargo_web_snippet_d2dec64de572d6c530f2d0f636aa3c79474eefd8": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).shiftKey;})());
            },
            "__cargo_web_snippet_dc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf": function() {
                console.error( 'Encountered a panic!' );
            },
            "__cargo_web_snippet_dcbfa3eb1cc89d9842b0ad8d9030a57a7cae7124": function($0) {
                return (Module.STDWEB_PRIVATE.acquire_js_reference( $0 ) instanceof Uint8Array) | 0;
            },
            "__cargo_web_snippet_e433235cb851604e5f84ff6d9c7a20ca1594e807": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);var ctx=Module.gl.get(($0));ctx.drawArrays(($1),0,($2));
            },
            "__cargo_web_snippet_e741b9d9071097746386b2c2ec044a2bc73e688c": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).appendChild(($1));
            },
            "__cargo_web_snippet_e9638d6405ab65f78daf4a5af9c9de14ecf1e2ec": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);Module.STDWEB_PRIVATE.unregister_raw_value(($0));
            },
            "__cargo_web_snippet_ea0de90bd9733ee43b06409f4de3d0d9167bdacc": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);var oReq=new XMLHttpRequest();var filename=($0);oReq.open("GET",filename,true);oReq.responseType="arraybuffer";var on_error_js=function(s){var on_error=($1);on_error(s);on_error.drop();};oReq.onload=function(oEvent){var status=oReq.status;var arrayBuffer=oReq.response;if(status==200&&arrayBuffer){var on_get_buffer=($2);on_get_buffer(new Uint8Array(arrayBuffer));on_get_buffer.drop();}else{on_error_js("Fail to get array buffer from network..");}};oReq.onerror=function(oEvent){on_error_js("Fail to read from network..");};oReq.send(null);
            },
            "__cargo_web_snippet_ecb854cd6ac4c4f3e6e3dbee517c287cca696b5b": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){return Module.gl.version;})());
            },
            "__cargo_web_snippet_ecbb5699d896b5d342fad8273d78b3433cd42281": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof MouseEvent && o.type === "mouseup");
            },
            "__cargo_web_snippet_ed3b0df31fcda5d490f9971007b9a94d968da39f": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);var ctx=Module.gl.get(($0));var h=Module.gl.get(($1));ctx.linkProgram(h.prog);var result=ctx.getProgramParameter(h.prog,ctx.LINK_STATUS);if(! result){console.log("ERROR while linking program :");console.log(ctx.getProgramInfoLog(h.prog));}
            },
            "__cargo_web_snippet_ef16c0b82c56f218e9b770adff637719c1875461": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){var ctx=Module.gl.get(($1));return Module.gl.add(ctx.createTexture());})());
            },
            "__cargo_web_snippet_ef51afd0f3a102a15e9d3ff1a75dfb448bf1239d": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);var ctx=Module.gl.get(($0));var h=Module.gl.get(($1));var shader=Module.gl.get(($2));ctx.attachShader(h.prog,shader)
            },
            "__cargo_web_snippet_f0adb99d5f5472bc041daf37d607ba384d5caab7": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).getBoundingClientRect().left;})());
            },
            "__cargo_web_snippet_f1c5b555b7858c4f021b91769dce6f5bafdef9a2": function($0, $1, $2, $3) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){var callback=($1);var request=($2).requestAnimationFrame(callback);return{request:request,callback:callback,window:($3)};})());
            },
            "__cargo_web_snippet_f70510c74a958be1c72bdf281ae422b5c002b5da": function($0, $1, $2) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);Module.STDWEB_PRIVATE.from_js($0, (function(){var gl=(($1)).getContext("webgl2",{alpha:false,preserveDrawingBuffer:true});var version=2;if(! gl){gl=(($2)).getContext("webgl",{alpha:false,preserveDrawingBuffer:true});version=1;}var ext=gl.getExtension("WEBGL_depth_texture");if(! Module.gl){Module.gl={};Module.gl.counter=1;Module.gl.version=version;Module.gl.matrix4x4=new Float32Array([1.0,0,0,0,0,1.0,0.0,0,0,0,1.0,0,0,0,0,1.0]);Module.gl.pool={};Module.gl.get=function(id){return Module.gl.pool[id];};Module.gl.add=function(o){var c=Module.gl.counter;Module.gl.pool[c]=o;Module.gl.counter+=1;return c;};Module.gl.remove=function(id){delete Module.gl.pool[id];return c;};console.log("opengl "+gl.getParameter(gl.VERSION));console.log("shading language "+gl.getParameter(gl.SHADING_LANGUAGE_VERSION));console.log("vendor "+gl.getParameter(gl.VENDOR));}return Module.gl.add(gl);})());
            },
            "__cargo_web_snippet_f83cc7a1ce50973ffea1ecc8ec793fc6bc866b4c": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){{var ctx=Module.gl.get(($1));return Module.gl.add(ctx.createBuffer());}})());
            },
            "__cargo_web_snippet_fb9c40921e38f97e000d9581e2677b4b4c3140ae": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);var ctx=Module.gl.get(($0));var loc=Module.gl.get(($1));ctx.uniform1i(loc,($2))
            },
            "__cargo_web_snippet_fc18467ee7b1f9a5e6e811e07f6fff2c88679ff3": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).code;})());
            },
            "__cargo_web_snippet_fd2fcf033f3db2446e9c8d4c5d474b58f9c4a759": function($0, $1, $2, $3, $4, $5, $6, $7, $8) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);$5 = Module.STDWEB_PRIVATE.to_js($5);$6 = Module.STDWEB_PRIVATE.to_js($6);$7 = Module.STDWEB_PRIVATE.to_js($7);$8 = Module.STDWEB_PRIVATE.to_js($8);var realToCSSPixels=window.devicePixelRatio;(($0)).width=($1)*realToCSSPixels;(($2)).height=($3)*realToCSSPixels;(($4)).style.width=($5)+"px";(($6)).style.height=($7)+"px";($8).tabIndex=1;
            },
            "__cargo_web_snippet_fe6996af9afcedfc5cdc9a1825610594eb090210": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);var ctx=Module.gl.get(($0));ctx.blendEquation(($1));
            },
            "__cargo_web_snippet_ff5103e6cc179d13b4c7a785bdce2708fd559fc0": function($0) {
                Module.STDWEB_PRIVATE.tmp = Module.STDWEB_PRIVATE.to_js( $0 );
            },
                "__web_on_grow": __web_on_grow
            }
        },
        initialize: function( instance ) {
            Object.defineProperty( Module, 'instance', { value: instance } );
            Object.defineProperty( Module, 'web_malloc', { value: Module.instance.exports.__web_malloc } );
            Object.defineProperty( Module, 'web_free', { value: Module.instance.exports.__web_free } );
            Object.defineProperty( Module, 'web_table', { value: Module.instance.exports.__indirect_function_table } );

            
            __web_on_grow();
            Module.instance.exports.main();

            return Module.exports;
        }
    };
}
 ));
}));
