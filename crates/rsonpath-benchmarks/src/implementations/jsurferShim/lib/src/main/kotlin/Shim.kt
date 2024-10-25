package com.v0ldek.rsonpath.jsurferShim

import java.nio.charset.Charset
import java.nio.charset.StandardCharsets
import java.nio.file.Files
import java.nio.file.Paths
import org.jsfr.json.*
import org.jsfr.json.compiler.JsonPathCompiler

class JsonFile(val contents: String)

fun interface CompiledQuery {
    fun run(file: JsonFile): Long
}

object Shim {
    private fun readFile(path: String, encoding: Charset): String =
            Files.readString(Paths.get(path), encoding)

    @JvmStatic
    fun loadFile(filePath: String): JsonFile {
        val json = readFile(filePath, StandardCharsets.UTF_8)
        return JsonFile(json)
    }

    @JvmStatic
    fun compileQuery(query: String): CompiledQuery {
        var result = 0L
        val surfer = JsonSurferFastJson.INSTANCE
        val compiledPath = JsonPathCompiler.compile(query)
        val config =
                surfer.configBuilder()
                        .bind(compiledPath, JsonPathListener { _, _ -> result += 1L })
                        .build()
        return CompiledQuery { file ->
            surfer.surf(file.contents, config)
            result
        }
    }

    @JvmStatic
    fun overheadShim(): CompiledQuery {
        return CompiledQuery { _ -> 0 }
    }
}
