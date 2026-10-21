<!--
  One task (`/plans/[pid]/tasks/[tpid]`): what it is and where it stands, and its links, such as the Jira or Linear
  issue it is tracked as. The service stores a link's address and never fetches it.
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type Task } from "#lib/api/ppm.js";
  import LinksPanel from "#lib/components/LinksPanel.svelte";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  const tpid = $derived(page.params.tpid ?? "");
  let task = $state<Task | null>(null);
  let error = $state<string | null>(null);

  const STATUS: Record<Task["status"], string> = {
    get todo() { return t("p.task.s_todo"); },
    get in_progress() { return t("p.task.s_in_progress"); },
    get in_review() { return t("p.task.s_in_review"); },
    get done() { return t("p.task.s_done"); },
    get blocked() { return t("p.task.s_blocked"); },
  };

  onMount(async () => {
    try {
      task = await ppm.getTask(pid, tpid);
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.task.could_not_load");
    }
  });
</script>

<svelte:head><title>{t("p.task.task_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}/wbs`)}>{t("p.task.back")}</a></p>
<h1>{task ? task.title : t("p.task.task")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
{#if task}
  <dl>
    <dt>{t("p.task.status")}</dt><dd>{STATUS[task.status] ?? task.status}</dd>
    {#if task.due_date}<dt>{t("p.task.due")}</dt><dd>{task.due_date}</dd>{/if}
    {#if task.assignee_ref}<dt>{t("p.task.assignee")}</dt><dd>{task.assignee_ref}</dd>{/if}
    {#if task.description}<dt>{t("p.task.description")}</dt><dd>{task.description}</dd>{/if}
  </dl>
  <p class="small muted">{t("p.task.links_intro")}</p>
  <LinksPanel {pid} taskPid={tpid} />
{:else if !error}
  <p class="muted">{t("p.task.loading")}</p>
{/if}
