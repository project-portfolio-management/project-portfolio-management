// MA-2: the tokens, encrypted with an AES-GCM key that lives in the Android Keystore and never leaves it. The ciphertext is
// kept in a private SharedPreferences file, which backup excludes. A value that cannot be decrypted (a key lost with a
// reinstall or a lock-screen change) is forgotten, and the person signs in again.
package io.github.projectportfoliomanagement.mobile.app

import android.content.Context
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import android.util.Base64
import androidx.core.content.edit
import io.github.projectportfoliomanagement.mobile.api.WireJson
import io.github.projectportfoliomanagement.mobile.auth.TokenStore
import io.github.projectportfoliomanagement.mobile.auth.Tokens
import java.security.GeneralSecurityException
import java.security.KeyStore
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec

class KeystoreTokenStore(context: Context) : TokenStore {
    private val prefs = context.getSharedPreferences("tokens", Context.MODE_PRIVATE)

    override fun load(): Tokens? {
        val iv = prefs.getString(IV, null) ?: return null
        val sealed = prefs.getString(DATA, null) ?: return null
        return try {
            val cipher = Cipher.getInstance(TRANSFORMATION)
            cipher.init(Cipher.DECRYPT_MODE, key(), GCMParameterSpec(128, Base64.decode(iv, Base64.NO_WRAP)))
            val json = String(cipher.doFinal(Base64.decode(sealed, Base64.NO_WRAP)), Charsets.UTF_8)
            WireJson.decodeFromString(Tokens.serializer(), json)
        } catch (_: GeneralSecurityException) {
            clear()
            null
        } catch (_: IllegalArgumentException) {
            clear()
            null
        }
    }

    override fun save(tokens: Tokens) {
        val cipher = Cipher.getInstance(TRANSFORMATION)
        cipher.init(Cipher.ENCRYPT_MODE, key())
        val sealed = cipher.doFinal(WireJson.encodeToString(Tokens.serializer(), tokens).toByteArray(Charsets.UTF_8))
        prefs.edit {
            putString(IV, Base64.encodeToString(cipher.iv, Base64.NO_WRAP))
            putString(DATA, Base64.encodeToString(sealed, Base64.NO_WRAP))
        }
    }

    override fun clear() {
        prefs.edit { clear() }
    }

    private fun key(): SecretKey {
        val keyStore = KeyStore.getInstance(KEYSTORE).apply { load(null) }
        (keyStore.getKey(ALIAS, null) as? SecretKey)?.let { return it }
        val generator = KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, KEYSTORE)
        generator.init(
            KeyGenParameterSpec.Builder(ALIAS, KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT)
                .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                .setKeySize(256)
                .build(),
        )
        return generator.generateKey()
    }

    private companion object {
        const val KEYSTORE = "AndroidKeyStore"
        const val ALIAS = "ppm-tokens"
        const val TRANSFORMATION = "AES/GCM/NoPadding"
        const val IV = "iv"
        const val DATA = "tokens"
    }
}
