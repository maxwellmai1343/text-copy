<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  interface TextItem {
    id: number;
    content: string;
    created_at: string;
  }

  let texts: TextItem[] = [];
  let newText = '';
  let editingId: number | null = null;
  let editContent = '';
  let copiedId: number | null = null;
  let toast = '';

  async function loadTexts() {
    try {
      texts = await invoke<TextItem[]>('load_texts');
    } catch (error) {
      console.error('加载文本失败:', error);
    }
  }

  function showToast(message: string) {
    toast = message;
    setTimeout(() => toast = '', 2000);
  }

  async function handleAddText() {
    if (!newText.trim()) return;
    
    try {
      const newItem = await invoke<TextItem>('add_text', { content: newText });
      texts = [...texts, newItem];
      newText = '';
      showToast('添加成功');
    } catch (error) {
      console.error('添加文本失败:', error);
    }
  }

  async function handleDeleteText(id: number) {
    try {
      await invoke('delete_text', { id });
      texts = texts.filter(t => t.id !== id);
      showToast('已删除');
    } catch (error) {
      console.error('删除文本失败:', error);
    }
  }

  function startEditing(item: TextItem) {
    editingId = item.id;
    editContent = item.content;
  }

  async function handleUpdateText() {
    if (!editContent.trim() || editingId === null) return;
    
    try {
      const updatedItem = await invoke<TextItem>('update_text', { 
        id: editingId, 
        content: editContent 
      });
      texts = texts.map(t => t.id === editingId ? updatedItem : t);
      editingId = null;
      editContent = '';
      showToast('修改成功');
    } catch (error) {
      console.error('修改文本失败:', error);
    }
  }

  async function handleCopyText(item: TextItem) {
    try {
      await navigator.clipboard.writeText(item.content);
      copiedId = item.id;
      showToast('已复制到剪贴板！');
      setTimeout(() => copiedId = null, 1500);
    } catch (error) {
      console.error('复制失败:', error);
    }
  }

  loadTexts();
</script>

<div class="container">
  {#if toast}
    <div class="toast">{toast}</div>
  {/if}
  
  <div class="header">
    <h1>📋 文本复制助手</h1>
    <span>共 {texts.length} 条</span>
  </div>

  <div class="add-form">
    <input
      type="text"
      bind:value={newText}
      placeholder="输入要保存的文本..."
      class="add-input"
      on:keydown={(e) => e.key === 'Enter' && handleAddText()}
    />
    <button class="btn btn-primary" on:click={handleAddText}>
      添加
    </button>
  </div>

  {#if editingId !== null}
    <div class="edit-form">
      <h3>编辑文本</h3>
      <textarea
        bind:value={editContent}
        class="edit-textarea"
      ></textarea>
      <div class="edit-actions">
        <button class="btn btn-primary" on:click={handleUpdateText}>
          保存
        </button>
        <button 
          class="btn btn-secondary" 
          on:click={() => {
            editingId = null;
            editContent = '';
          }}
        >
          取消
        </button>
      </div>
    </div>
  {/if}

  <div class="text-list">
    {#if texts.length === 0}
      <div class="empty-state">
        <p>还没有文本，添加一些文本吧！</p>
      </div>
    {:else}
      {#each [...texts].reverse() as item (item.id)}
        <div 
          class="text-item {copiedId === item.id ? 'copied' : ''}"
          role="button"
          tabindex="0"
          on:click={() => handleCopyText(item)}
          on:keydown={(e) => (e.key === 'Enter' || e.key === ' ') && handleCopyText(item)}
        >
          <div class="text-content">{item.content}</div>
          <div class="text-actions">
            <button 
              class="btn btn-secondary btn-sm"
              on:click|stopPropagation={() => startEditing(item)}
            >
              编辑
            </button>
            <button 
              class="btn btn-danger btn-sm"
              on:click|stopPropagation={() => handleDeleteText(item.id)}
            >
              删除
            </button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>
