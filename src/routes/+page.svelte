<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import type { PhotoMetadata, FolderAnalysis } from "../types";

  let photos = $state<PhotoMetadata[]>([]);
  let stats = $state<FolderAnalysis['stats'] | null>(null);
  let loading = $state(false);
  let error = $state<string | null>(null)
  


  async function selectFolder() {
    const folder = await open({
      directory: true,
    });
    if (!folder) return;

    loading = true;
    error = null;
    
    try {
      const result = await invoke<FolderAnalysis>("analyze_folder", { folderPath: folder as string });
      photos = result.photos;
      stats = result.stats;
      console.log(`${photos.length} photos found in folder ${folder}`);
      console.log("Stats:", stats);
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error("Error reading folder:", error);
    } finally {
      loading = false;
    }
  }

  async function selectFile() {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'Images',
          extensions: ['jpg', 'jpeg', 'png', 'arw', 'cr3', 'nef', 'dng']
        },
      ],
    });

    if (selected === null) return; //cancel file selection

    loading = true;
    error = null;
    
    try {
      const meta = await invoke<PhotoMetadata>("read_exif_from_file", { filepath: selected as string });
      console.log(meta);
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error("Error reading file:", error);
    } finally {
      loading = false;
    }
  }
</script>

<main class="container">
  <h1>Welcome to Exif Reader and analysis</h1>

  <button onclick={selectFile}>Select a file</button>
  <button onclick={selectFolder}>Select a folder</button>
  {#if loading}
    <p>Loading...</p>
  {/if}
  {#if error}
    <p style="color: red;">Error: {error}</p>
  {/if}

  {#if photos.length > 0}
    <h2>Photos found: {photos.length}</h2>
  {/if}

  {#each photos as photo}
    <div style="margin-top: 1em; padding: 1em; border: 1px solid #ccc; border-radius: 8px; display: flex; gap: 1em; align-items: flex-start; text-align: left;">
      <img src={photo.thumbnail} alt={photo.filename} style="width: 160px; height: 120px; object-fit: cover; border-radius: 4px; flex-shrink: 0;" />
      <div>
        <h2 style="margin: 0 0 0.5em">{photo.filename}</h2>
        <p><strong>Camera:</strong> {photo.make} {photo.model}</p>
        <p><strong>Focal Length:</strong> {photo.focal_length}mm</p>
        <p><strong>Exposure:</strong> 1/{Math.round(1 / photo.shutter)}s at f/{photo.aperture}</p>
        <p><strong>ISO:</strong> {photo.iso_speed}</p>
        <p><strong>Date:</strong> {photo.datetime ? new Date(photo.datetime.replace(/^(\S+) (\S+) (\S+)$/, '$1T$2$3')).toLocaleString() : 'Unknown'}</p>
      </div>
    </div>
  {/each}
</main>

<style>

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}


@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
