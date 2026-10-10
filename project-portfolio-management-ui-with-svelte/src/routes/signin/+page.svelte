<script lang="ts">
  import type { ActionData } from "./$types";
  import { enhance } from "$app/forms";
  import { page } from "$app/state";
  import { PUBLIC_OIDC_SIGNIN_ENABLED, PUBLIC_OIDC_DIRECT_ENABLED, PUBLIC_OIDC_DIRECT_PROVIDER } from '$app/env/public';
  import { i18n, t, localeHref } from "#lib/i18n.svelte.js";

  let { form }: { form: ActionData } = $props();
  // Opt-in OIDC "Sign in with SSO": a plain <a> (browser navigation), shown
  // only when PUBLIC_OIDC_SIGNIN_ENABLED=true. Magic link stays the default.
  const ssoEnabled = PUBLIC_OIDC_SIGNIN_ENABLED === "true";
  // Direct OpenID Connect sign-in (HD-12): the BFF is the OIDC client (Microsoft Entra ID,
  // or a Keycloak realm). A plain <a>, because a form post would hit `form-action 'self'`.
  const oidcEnabled = PUBLIC_OIDC_DIRECT_ENABLED === "true";
  const oidcProvider = PUBLIC_OIDC_DIRECT_PROVIDER || "Microsoft Entra ID";
  const oidcError = $derived(page.url.searchParams.get("error"));
</script>

<!--
  Sign-in page (BFF, per-app magic-link login). Posts to the `default`
  server action, which calls the authentication service server-side with a
  return URL pointing back at THIS app's /verify. No token is held in the
  browser. (Copy uses plain English; full i18n for the signin/verify pages
  is a follow-up — the entity apps had no login UI before the BFF.)
-->
<svelte:head><title>{t("p.signin.sign_in")}</title></svelte:head>

<h1>{t("p.signin.sign_in")}</h1>

{#if form?.sent}
    <p class="banner">{t("p.signin.check_your_email_for_a")}</p>
{:else}
    <form class="stack" method="POST" use:enhance>
        <label>
            {t("p.signin.email")}
            <input type="email" name="email" required autocomplete="email" />
        </label>
        <input type="hidden" name="locale" value={i18n.locale} />
        <button class="button primary" type="submit">{t("p.signin.send_magic_link")}</button>
        {#if form?.error}
            <p class="banner" role="alert">{t("p.signin.could_not_send_the_sign")}</p>
        {/if}
    </form>
{/if}
{#if oidcError}
    <p class="banner" role="alert" data-testid="oidc-error">
        {oidcError === "oidc_denied"
            ? t("signin.oidcDenied")
            : oidcError === "oidc_unavailable"
              ? t("signin.oidcUnavailable")
              : t("signin.oidcError")}
    </p>
{/if}
{#if oidcEnabled}
    <p>
        <a class="button" href={localeHref("/signin/oidc")} data-sveltekit-reload>{t("signin.oidc", { provider: oidcProvider })}</a>
    </p>
{/if}
{#if ssoEnabled}
    <p>
        <a class="button" href={localeHref("/signin/sso")}>{t("signin.sso")}</a>
    </p>
{/if}
