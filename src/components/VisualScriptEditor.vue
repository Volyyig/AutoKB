<template>
  <div class="visual-editor fixed inset-0 z-[1000] bg-background-light dark:bg-background-dark flex flex-col font-display antialiased overflow-hidden h-screen">
    <!-- Top Navigation Bar -->
    <header class="flex h-14 items-center justify-between border-b border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 px-4 shrink-0 z-50">
      <div class="flex items-center gap-4">
        <button @click="$emit('close')" class="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg text-slate-500 transition-colors">
          <span class="material-symbols-outlined">arrow_back</span>
        </button>
        <div class="flex items-center gap-2">
          <span class="material-symbols-outlined text-primary text-3xl">auto_fix_high</span>
          <h2 class="text-lg font-bold tracking-tight">AutoFlow <span class="text-slate-400 font-normal">脚本编辑器</span></h2>
        </div>
      </div>
      <div class="flex items-center gap-3">
        <div class="flex bg-slate-100 dark:bg-slate-800 p-1 rounded-lg mr-2">
          <button @click="loadCurrentRecording" :disabled="!store.hasEvents" class="flex items-center gap-1.5 px-3 py-1.5 rounded-md hover:bg-white dark:hover:bg-slate-700 text-sm font-medium transition-all">
            <span class="material-symbols-outlined text-sm">history</span> 导入录制
          </button>
          <button @click="loadIntoPlayback" :disabled="!currentScript" class="flex items-center gap-1.5 px-3 py-1.5 rounded-md hover:bg-white dark:hover:bg-slate-700 text-sm font-medium transition-all">
            <span class="material-symbols-outlined text-sm">play_arrow</span> 测试运行
          </button>
        </div>
        <button @click="saveChanges" :disabled="!currentScript" class="flex items-center gap-2 bg-primary text-white px-4 py-2 rounded-lg text-sm font-bold hover:bg-primary/90 transition-colors shadow-sm shadow-primary/20">
          <span class="material-symbols-outlined text-sm">save</span> {{ currentScriptPath ? '保存修改' : '保存为新脚本' }}
        </button>
      </div>
    </header>

    <!-- Main Workspace -->
    <main class="flex-1 flex overflow-hidden">
      <!-- Sidebar: Library & Saved Scripts -->
      <aside class="w-64 border-r border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 flex flex-col shrink-0">
        <div class="p-4 border-b border-slate-100 dark:border-slate-800">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-xs font-bold text-slate-500 uppercase tracking-wider">已存脚本</h3>
            <button @click="addNewScript" class="p-1 hover:bg-slate-100 dark:hover:bg-slate-800 rounded text-primary">
              <span class="material-symbols-outlined text-sm">add</span>
            </button>
          </div>
          <div class="relative">
            <span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 text-lg">search</span>
            <input class="w-full pl-10 pr-4 py-2 bg-slate-50 dark:bg-slate-800 border-none rounded-lg text-sm focus:ring-2 focus:ring-primary/20" placeholder="搜索脚本..." type="text"/>
          </div>
        </div>
        <div class="flex-1 overflow-y-auto p-2 space-y-1">
          <div v-for="script in store.savedScripts" :key="script.path" 
               @click="loadScriptForEdit(script.path)"
               :class="['group flex items-center justify-between p-2.5 rounded-lg cursor-pointer transition-all', 
                        currentScriptPath === script.path ? 'bg-primary/5 text-primary' : 'hover:bg-slate-50 dark:hover:bg-slate-800 text-slate-600 dark:text-slate-400']">
            <div class="flex items-center gap-2 overflow-hidden">
              <span class="material-symbols-outlined text-lg opacity-60">description</span>
              <span class="text-sm font-medium truncate">{{ script.name }}</span>
            </div>
            <button @click.stop="deleteScript(script.path)" class="p-1 opacity-0 group-hover:opacity-100 hover:bg-red-50 dark:hover:bg-red-900/20 text-red-500 rounded transition-all">
              <span class="material-symbols-outlined text-sm">delete</span>
            </button>
          </div>
          <div v-if="store.savedScripts.length === 0" class="p-4 text-center text-xs text-slate-400">
            暂无已保存脚本
          </div>
        </div>
      </aside>

      <!-- Editor Content -->
      <section class="flex-1 bg-background-light dark:bg-background-dark relative overflow-hidden flex flex-col">
        <div v-if="!currentScript" class="flex-1 flex flex-col items-center justify-center text-slate-400 opacity-60">
          <span class="material-symbols-outlined text-6xl mb-4">edit_square</span>
          <p>请选择脚本进行编辑或新建</p>
        </div>
        <template v-else>
          <!-- Canvas Toolbar (Simplified) -->
          <div class="absolute top-4 left-1/2 -translate-x-1/2 flex items-center gap-3 bg-white dark:bg-slate-900 shadow-xl border border-slate-200 dark:border-slate-700 rounded-full px-4 py-2 z-20">
            <div class="flex items-center gap-2">
              <input v-model="currentScript.name" class="bg-transparent border-none p-0 text-sm font-bold focus:ring-0 w-32" placeholder="脚本名称..." />
              <div class="w-px h-4 bg-slate-200 dark:bg-slate-700 mx-1"></div>
              <span class="text-xs font-medium text-slate-400">{{ currentScript.events.length }} 个节点</span>
            </div>
            <div class="flex items-center gap-1 relative">
              <button @click="showAddMenu = !showAddMenu" class="p-1.5 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full text-primary transition-colors">
                <span class="material-symbols-outlined">add_circle</span>
              </button>
              <div v-if="showAddMenu" class="absolute top-full mt-2 left-0 w-48 bg-white dark:bg-slate-900 shadow-2xl border border-slate-200 dark:border-slate-800 rounded-xl p-2 z-[100]">
                <button v-for="t in ['Delay', 'KeyPress', 'KeyRelease', 'MousePress', 'MouseRelease', 'MouseMove']" :key="t" 
                        @click="addEventTemplate(t)" class="w-full text-left p-2 hover:bg-slate-50 dark:hover:bg-slate-800 rounded-lg text-sm flex items-center gap-2">
                   <span class="material-symbols-outlined text-sm opacity-60">{{ getIconForType(t) }}</span>
                   {{ formatGroupTitle(t) }}
                </button>
              </div>
            </div>
          </div>

          <!-- Canvas Content: List of Event Groups -->
          <div class="flex-1 overflow-y-auto p-20 py-16 scroll-smooth canvas-grid">
            <div class="max-w-xl mx-auto flex flex-col items-center space-y-8">
              <!-- Start Node -->
              <div class="flex flex-col items-center">
                <div class="w-40 h-14 bg-white dark:bg-slate-800 border-2 border-primary rounded-xl flex items-center justify-center shadow-lg">
                  <span class="material-symbols-outlined text-primary mr-2">play_circle</span>
                  <span class="font-bold text-sm">流程开始</span>
                </div>
                <div class="w-0.5 h-8 bg-slate-300 dark:bg-slate-700 mt-2"></div>
              </div>

              <!-- Event Groups -->
              <div v-for="(group, index) in groups" :key="index" class="w-full flex flex-col items-center">
                <div @click="toggleGroup(index)" 
                     :class="['w-full bg-white dark:bg-slate-800 border rounded-xl shadow-sm hover:shadow-md transition-all cursor-pointer overflow-hidden group',
                              group.expanded ? 'ring-2 ring-primary/20 border-primary/50' : 'border-slate-200 dark:border-slate-800']">
                  <!-- Group Header -->
                  <div class="p-4 flex items-center justify-between bg-slate-50/50 dark:bg-slate-800/50">
                    <div class="flex items-center gap-3">
                      <span class="material-symbols-outlined text-primary p-1.5 bg-primary/10 rounded-lg transition-transform group-hover:scale-110">{{ getIconForType(group.type) }}</span>
                      <div>
                        <div class="flex items-center gap-2">
                          <span class="text-sm font-bold">{{ formatGroupTitle(group.type) }}</span>
                          <span class="text-[10px] px-1.5 py-0.5 bg-slate-200 dark:bg-slate-700 rounded-full font-bold uppercase">{{ group.events.length }} 步骤</span>
                        </div>
                        <p class="text-[10px] text-slate-400 mt-0.5">连续执行 {{ group.events.length }} 个同类动作</p>
                      </div>
                    </div>
                    <div class="flex items-center gap-2">
                      <button @click.stop="deleteGroup(index)" class="p-1.5 hover:bg-red-50 dark:hover:bg-red-900/20 text-red-500 rounded-lg opacity-0 group-hover:opacity-100 transition-all">
                        <span class="material-symbols-outlined text-lg">delete</span>
                      </button>
                      <span class="material-symbols-outlined text-slate-400 transition-transform" :class="{ 'rotate-180': group.expanded }">expand_more</span>
                    </div>
                  </div>

                  <!-- Group Details -->
                  <div v-if="group.expanded" class="p-4 space-y-3 border-t border-slate-100 dark:border-slate-800">
                    <div v-for="(event, eIndex) in (group.events as any[])" :key="eIndex" class="flex items-center justify-between gap-4 p-2 bg-slate-50 dark:bg-slate-900/50 rounded-lg border border-transparent hover:border-slate-200 dark:hover:border-slate-700 transition-colors">
                      <div class="flex items-center gap-3 overflow-hidden flex-1">
                        <span class="text-xs text-slate-400 font-mono w-4 shrink-0">{{ eIndex + 1 }}</span>
                        <!-- Custom settings per event type -->
                        <div class="flex-1 flex items-center gap-2">
                          <template v-if="event.event_type === 'Delay'">
                            <input type="number" v-model.number="event.duration_ms" class="w-20 px-2 py-1 text-xs bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded focus:ring-1 focus:ring-primary outline-none" min="0">
                            <span class="text-xs text-slate-400">ms</span>
                          </template>
                          <template v-else-if="event.event_type === 'KeyPress' || event.event_type === 'KeyRelease'">
                             <button @click="startCapture(index, eIndex)" 
                                     :class="['px-3 py-1 text-xs font-mono rounded border border-dashed transition-colors',
                                              capturingIndex?.groupIndex === index && capturingIndex?.eventIndex === eIndex ? 'bg-primary/10 border-primary text-primary' : 'bg-white dark:bg-slate-800 border-slate-300 dark:border-slate-600']">
                                {{ capturingIndex?.groupIndex === index && capturingIndex?.eventIndex === eIndex ? '等待输入...' : getKeyDisplay(event.key) }}
                             </button>
                          </template>
                          <template v-else-if="event.event_type.startsWith('Mouse')">
                            <span class="text-xs text-slate-400">X:</span>
                            <input type="number" v-model.number="event.x" class="w-14 px-1 py-0.5 text-xs bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded" />
                            <span class="text-xs text-slate-400 ml-1">Y:</span>
                            <input type="number" v-model.number="event.y" class="w-14 px-1 py-0.5 text-xs bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded" />
                          </template>
                        </div>
                        <span class="text-[10px] text-slate-400 truncate opacity-40 hidden md:block">{{ getEventDescription(event) }}</span>
                      </div>
                      <button @click.stop="deleteEvent(index, eIndex)" class="p-1 hover:text-red-500 transition-colors shrink-0">
                        <span class="material-symbols-outlined text-sm">close</span>
                      </button>
                    </div>
                  </div>
                </div>
                <!-- Connector arrow (if not last) -->
                <div v-if="index < groups.length - 1" class="w-0.5 h-8 bg-slate-300 dark:bg-slate-700 my-1"></div>
              </div>

              <!-- End Node -->
              <div class="flex flex-col items-center">
                <div class="w-0.5 h-8 bg-slate-300 dark:bg-slate-700 my-1"></div>
                <div class="w-32 h-10 bg-slate-100 dark:bg-slate-800 border-2 border-slate-300 dark:border-slate-700 rounded-lg flex items-center justify-center text-slate-400">
                  <span class="material-symbols-outlined text-sm mr-2">logout</span>
                  <span class="font-bold text-xs uppercase tracking-widest">结束</span>
                </div>
              </div>
            </div>
          </div>
        </template>

        <!-- Breadcrumbs -->
        <div class="h-10 border-t border-slate-200 dark:border-slate-800 bg-white/80 dark:bg-slate-900/80 backdrop-blur-sm px-6 flex items-center gap-3 text-[10px] font-bold text-slate-500 uppercase tracking-widest z-20">
          <span>工作流库</span>
          <span class="material-symbols-outlined text-[10px]">chevron_right</span>
          <span class="text-primary">{{ currentScript?.name || '新脚本' }}</span>
        </div>
      </section>

      <!-- Right Sidebar: Properties Configuration (ParamEditor integrated) -->
      <aside v-if="currentScript" class="w-72 border-l border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 flex flex-col shrink-0">
        <div class="p-4 border-b border-slate-100 dark:border-slate-800 flex items-center justify-between shadow-sm">
          <h3 class="font-bold text-sm">全局属性</h3>
          <span class="text-[10px] bg-primary/10 text-primary px-2 py-0.5 rounded-full font-bold uppercase">Script Config</span>
        </div>
        <div class="flex-1 overflow-y-auto p-4 space-y-6">
          <div class="space-y-2">
            <label class="text-[10px] font-black text-slate-400 uppercase tracking-wider">脚本描述</label>
            <textarea v-model="currentScript.description" class="w-full text-xs bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg px-3 py-2 h-24 outline-none focus:ring-1 focus:ring-primary transition-all" placeholder="简述该流程的作用..."></textarea>
          </div>
          
          <div class="space-y-4">
            <label class="text-[10px] font-black text-slate-400 uppercase tracking-wider">执行模式</label>
            <div class="space-y-4">
              <div class="space-y-1.5">
                <div class="flex items-center justify-between text-xs">
                  <span class="text-slate-500">执行速度</span>
                  <span class="font-mono font-bold text-primary">{{ currentScript.speed_multiplier }}x</span>
                </div>
                <input type="range" v-model.number="currentScript.speed_multiplier" min="0.1" max="5.0" step="0.1" class="w-full h-1.5 bg-slate-100 dark:bg-slate-700 rounded-lg appearance-none cursor-pointer accent-primary" />
              </div>
              <div class="flex items-center justify-between">
                <span class="text-xs text-slate-500">循环次数 (0=无限)</span>
                <input type="number" v-model.number="currentScript.loop_config.count" class="w-20 px-2 py-1.5 text-xs bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg outline-none focus:ring-1 focus:ring-primary" />
              </div>
              <div class="flex items-center justify-between">
                <span class="text-xs text-slate-500">循环间隔 (ms)</span>
                <input type="number" v-model.number="currentScript.loop_config.delay_between_ms" class="w-20 px-2 py-1.5 text-xs bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg outline-none focus:ring-1 focus:ring-primary" />
              </div>
            </div>
          </div>

          <div class="p-4 bg-amber-50 dark:bg-amber-900/10 rounded-xl border border-amber-100 dark:border-amber-900/20">
             <div class="flex gap-2 items-start">
               <span class="material-symbols-outlined text-amber-500 text-lg">lightbulb</span>
               <div class="text-[10px] text-amber-700 dark:text-amber-300 leading-relaxed font-medium">
                 编辑器会自动将连续的同类动作（如多个按键）折叠为组，以保持流程整洁度。
               </div>
             </div>
          </div>
        </div>
        <div class="p-4 border-t border-slate-200 dark:border-slate-800 bg-slate-50/50 dark:bg-slate-900/50">
          <button @click="clearEvents" class="w-full py-2.5 bg-slate-100 dark:bg-slate-800 hover:bg-red-500 hover:text-white text-slate-500 rounded-lg text-[10px] font-black transition-all uppercase tracking-widest border border-transparent shadow-sm">
            清空所有节点
          </button>
        </div>
      </aside>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { useScriptStore } from '../stores/scriptStore';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import type { Script } from '../types/script';
import { getEventDescription, getKeyDisplay } from '../types/script';
import { groupEvents, formatGroupTitle, type EventGroup } from '../utils/eventGrouping';

const store = useScriptStore();
const currentScriptPath = ref<string | null>(null);
const currentScript = ref<Script | null>(null);
const groups = ref<EventGroup[]>([]);
const showAddMenu = ref(false);
const capturingIndex = ref<{ groupIndex: number, eventIndex: number } | null>(null);

defineEmits(['close']);

// Watch currentScript and build `groups` ref
watch(() => currentScript.value, (newScript) => {
    if (!newScript) {
        groups.value = [];
        return;
    }
    const expandedStates = groups.value.map(g => g.expanded);
    const newGroups = groupEvents(newScript.events);
    newGroups.forEach((group, i) => {
        if (expandedStates[i] !== undefined) {
            group.expanded = expandedStates[i];
        } else {
            group.expanded = true; // Default to expanded for visibility
        }
    });
    groups.value = newGroups;
}, { deep: true, immediate: true });

function toggleGroup(index: number) {
    if (groups.value[index]) {
        groups.value[index].expanded = !groups.value[index].expanded;
    }
}

function getIconForType(type: string) {
    const map: Record<string, string> = {
        'Delay': 'timer',
        'KeyPress': 'keyboard',
        'KeyRelease': 'keyboard',
        'MousePress': 'mouse',
        'MouseRelease': 'mouse',
        'MouseMove': 'near_me',
        'MouseScroll': 'expand',
    };
    return map[type] || 'settings';
}

// --- Editing Functions ---

async function deleteScript(path: string) {
    if (!confirm(`确定要彻底删除脚本吗？`)) return;
    try {
        await invoke('delete_script', { path });
        if (currentScriptPath.value === path) {
            currentScript.value = null;
            currentScriptPath.value = null;
        }
        await store.listSavedScripts();
        store.showNotification('删除成功。', 'success');
    } catch (e) {
        store.showNotification('删除失败: ' + e, 'error');
    }
}

function loadCurrentRecording() {
    currentScript.value = JSON.parse(JSON.stringify(store.currentScript));
    currentScriptPath.value = null;
    store.showNotification('已加载当前录制。', 'info');
}

function addNewScript() {
    currentScript.value = store.createNewDraftScript();
    currentScriptPath.value = null;
    store.showNotification('已创建新脚本载入。', 'info');
}

function addEventTemplate(type: string) {
    if (!currentScript.value) return;
    let event: any = { event_type: type };
    switch (type) {
        case 'Delay': event.duration_ms = 100; break;
        case 'KeyPress':
        case 'KeyRelease': event.key = { type: 'Char', value: 'a' }; break;
        case 'MousePress':
        case 'MouseRelease': 
            event.button = 'left'; event.x = 0; event.y = 0; break;
        case 'MouseMove': event.x = 0; event.y = 0; break;
        case 'MouseScroll': event.delta_x = 0; event.delta_y = 100; break;
    }
    currentScript.value.events.push(event);
    showAddMenu.value = false;
}

function startCapture(groupIndex: number, eventIndex: number) {
    capturingIndex.value = { groupIndex, eventIndex };
    window.addEventListener('keydown', onCaptureKeyDown, { once: true });
}

function onCaptureKeyDown(e: KeyboardEvent) {
    if (!capturingIndex.value || !currentScript.value) return;
    e.preventDefault();
    e.stopPropagation();
    const { groupIndex, eventIndex } = capturingIndex.value;
    const group = groups.value[groupIndex];
    if (group) {
        const realIndex = group.startIndex + eventIndex;
        const event = currentScript.value.events[realIndex];
        if (event && (event.event_type === 'KeyPress' || event.event_type === 'KeyRelease')) {
            if (e.key.length === 1) {
                event.key = { type: 'Char', value: e.key };
            } else {
                let keyName = e.key;
                 const keyMap: Record<string, string> = {
                    ' ': 'Space', 'Control': 'ControlLeft', 'Shift': 'ShiftLeft', 'Alt': 'Alt',
                    'Meta': 'MetaLeft', 'ArrowUp': 'UpArrow', 'ArrowDown': 'DownArrow',
                    'ArrowLeft': 'LeftArrow', 'ArrowRight': 'RightArrow', 'Enter': 'Return',
                    'Backspace': 'Backspace', 'Escape': 'Escape', 'Tab': 'Tab'
                };
                if (keyMap[keyName]) keyName = keyMap[keyName];
                event.key = { type: 'Special', value: keyName };
            }
        }
    }
    capturingIndex.value = null;
}

function deleteEvent(groupIndex: number, eventIndex: number) {
    if (!currentScript.value) return;
    const group = groups.value[groupIndex];
    currentScript.value.events.splice(group.startIndex + eventIndex, 1);
}

function deleteGroup(groupIndex: number) {
    if (!currentScript.value) return;
    if (!confirm('确定要删除整个组吗？')) return;
    const group = groups.value[groupIndex];
    currentScript.value.events.splice(group.startIndex, group.events.length);
}

function clearEvents() {
    if (currentScript.value && confirm('确定清空所有节点吗？')) {
        currentScript.value.events = [];
    }
}

async function refreshScripts() {
    await store.listSavedScripts();
}

async function loadScriptForEdit(path: string) {
    try {
        const script = await invoke<Script>('load_script', { path });
        currentScript.value = script;
        currentScriptPath.value = path;
    } catch (e) {
        console.error("Failed to load script", e);
    }
}

async function saveChanges() {
    if (!currentScript.value) return;
    try {
        let path = currentScriptPath.value;
        if (!path) {
            const defaultDir = await invoke<string>('get_scripts_dir');
            const savePath = await save({
                defaultPath: `${defaultDir}/${currentScript.value.name}.autokb`,
                filters: [{ name: 'AutoKB Script', extensions: ['autokb'] }],
            });
            if (!savePath) return;
            path = savePath;
            currentScriptPath.value = path;
        }
        await invoke('save_script', { script: currentScript.value, path });
        store.showNotification('保存成功！', 'success');
        await store.listSavedScripts();
    } catch (e) {
        store.showNotification('保存失败: ' + e, 'error');
    }
}

async function loadIntoPlayback() {
    if (!currentScript.value) return;
    store.currentScript = JSON.parse(JSON.stringify(currentScript.value));
    store.statusMessage = `已载入 ${currentScript.value.name}`;
    store.showNotification(`已载入脚本。`, 'info');
}

onMounted(() => refreshScripts());
</script>

<style scoped>
.canvas-grid {
  background-size: 20px 20px;
  background-image: radial-gradient(circle, #e2e8f0 1px, transparent 1px);
}
.dark .canvas-grid {
  background-image: radial-gradient(circle, #1e293b 1px, transparent 1px);
}

input[type="range"]::-webkit-slider-thumb {
  appearance: none;
  width: 14px;
  height: 14px;
  background: white;
  border: 2px solid theme('colors.primary');
  border-radius: 50%;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}
</style>
