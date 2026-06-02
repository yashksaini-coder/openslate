<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "$lib/api";
  import * as auth from "$lib/auth.svelte";
  import { goto } from "$app/navigation";

  type NoteSummary = {
    id: string;
    title: string;
    slug: string;
    tags: string[];
    created_at: string;
    updated_at: string;
  };

  type NoteDetail = NoteSummary & {
    content: string;
    backlinks: { title: string; slug: string }[];
  };

  let notes = $state<NoteSummary[]>([]);
  let selected = $state<NoteDetail | null>(null);
  let loading = $state(true);
  let editing = $state(false);
  let editTitle = $state("");
  let editContent = $state("");
  let editTags = $state("");

  onMount(() => {
    loadNotes();
  });

  async function loadNotes() {
    loading = true;
    try {
      const res = await api("/api/notes");
      notes = await res.json();
    } catch {
      notes = [];
    }
    loading = false;
  }

  async function selectNote(slug: string) {
    editing = false;
    const res = await api(`/api/notes/${slug}`);
    if (res.ok) {
      selected = await res.json();
    }
  }

  function startCreate() {
    editing = true;
    editTitle = "";
    editContent = "";
    editTags = "";
    selected = null;
  }

  function startEdit() {
    if (!selected) return;
    editing = true;
    editTitle = selected.title;
    editContent = selected.content;
    editTags = selected.tags.join(", ");
  }

  async function save() {
    if (!editing) return;
    const tags = editTags
      .split(",")
      .map((t) => t.trim())
      .filter(Boolean);

    if (selected?.slug && editTitle === selected.title) {
      // Update
      await api(`/api/notes/${selected.slug}`, {
        method: "PUT",
        body: JSON.stringify({
          title: editTitle,
          content: editContent,
          tags,
        }),
      });
    } else {
      // Create
      await api("/api/notes", {
        method: "POST",
        body: JSON.stringify({
          title: editTitle || "Untitled",
          content: editContent,
          tags,
        }),
      });
    }
    editing = false;
    await loadNotes();
  }

  async function del() {
    if (!selected) return;
    if (!confirm("Delete this note?")) return;
    await api(`/api/notes/${selected.slug}`, { method: "DELETE" });
    selected = null;
    await loadNotes();
  }

  async function handleLogout() {
    await auth.logout();
    goto("/login");
  }

  function formatDate(iso: string) {
    return iso.slice(0, 10);
  }
</script>

<div class="flex h-screen">
  <!-- Sidebar -->
  <aside class="w-64 border-r flex flex-col bg-gray-50">
    <div class="p-3 border-b flex items-center justify-between">
      <h1 class="font-bold text-lg">openslate</h1>
      <button onclick={handleLogout} class="text-xs text-red-500">Log out</button>
    </div>
    <button
      onclick={startCreate}
      class="mx-3 mt-2 rounded bg-black px-3 py-1.5 text-sm text-white hover:bg-gray-800"
    >
      + New note
    </button>
    <nav class="flex-1 overflow-y-auto p-2 space-y-1">
      {#if loading}
        <p class="text-sm text-gray-400 p-2">Loading...</p>
      {:else if notes.length === 0}
        <p class="text-sm text-gray-400 p-2">No notes yet</p>
      {:else}
        {#each notes as note}
          <button
            onclick={() => selectNote(note.slug)}
            class="w-full text-left p-2 rounded text-sm hover:bg-gray-200"
            class:bg-gray-200={selected?.slug === note.slug}
          >
            <div class="font-medium truncate">{note.title}</div>
            <div class="text-xs text-gray-400">{formatDate(note.updated_at)}</div>
            {#if note.tags.length > 0}
              <div class="flex gap-1 mt-1 flex-wrap">
                {#each note.tags as tag}
                  <span class="text-xs bg-gray-200 px-1.5 py-0.5 rounded">{tag}</span>
                {/each}
              </div>
            {/if}
          </button>
        {/each}
      {/if}
    </nav>
  </aside>

  <!-- Main area -->
  <main class="flex-1 flex flex-col p-4">
    {#if editing}
      <div class="space-y-3 flex-1 flex flex-col">
        <input
          bind:value={editTitle}
          placeholder="Note title"
          class="text-2xl font-bold border-b pb-2 outline-none"
        />
        <input
          bind:value={editTags}
          placeholder="Tags (comma separated)"
          class="text-sm text-gray-500 outline-none border-b pb-2"
        />
        <textarea
          bind:value={editContent}
          class="flex-1 resize-none outline-none font-mono text-sm"
          placeholder="Write in markdown..."
        ></textarea>
        <div class="flex gap-2">
          <button onclick={save} class="rounded bg-black px-4 py-2 text-sm text-white">
            Save
          </button>
          <button onclick={() => (editing = false)} class="rounded border px-4 py-2 text-sm">
            Cancel
          </button>
        </div>
      </div>
    {:else if selected}
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-2xl font-bold">{selected.title}</h2>
        <div class="flex gap-2">
          <button onclick={startEdit} class="text-sm text-blue-500">Edit</button>
          <button onclick={del} class="text-sm text-red-500">Delete</button>
        </div>
      </div>
      {#if selected.tags.length > 0}
        <div class="flex gap-1 mb-2">
          {#each selected.tags as tag}
            <span class="text-xs bg-gray-200 px-1.5 py-0.5 rounded">{tag}</span>
          {/each}
        </div>
      {/if}
      <div class="prose prose-sm max-w-none flex-1 overflow-y-auto">
        {#if selected.content}
          {selected.content}
        {:else}
          <p class="text-gray-400">No content</p>
        {/if}
      </div>
      {#if selected.backlinks.length > 0}
        <div class="border-t pt-2 mt-4">
          <p class="text-xs text-gray-500 mb-1">Linked from:</p>
          {#each selected.backlinks as bl}
            <button
              onclick={() => selectNote(bl.slug)}
              class="text-sm text-blue-500 hover:underline"
            >
              {bl.title}
            </button>
          {/each}
        </div>
      {/if}
    {:else}
      <div class="flex-1 flex items-center justify-center text-gray-400">
        <p>Select or create a note</p>
      </div>
    {/if}
  </main>
</div>
