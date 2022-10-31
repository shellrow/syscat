<script setup>
import {ref ,onMounted, onUnmounted} from 'vue';
import {debounce} from 'lodash';
import {Document, Menu as IconMenu, Setting, Sunny, Moon, Expand, Fold, Cpu, Odometer, List, Share} from '@element-plus/icons-vue';

const innerWidth = ref(window.innerWidth);
const innerHeight = ref(window.innerHeight);
const theme = ref('light');
const activeIndex = ref('0');
const mode = ref(true);
const isCollapse = ref(innerWidth.value < 1200 ? true : false);

if (localStorage.theme === 'dark') {
    document.documentElement.classList.add('dark');
    theme.value = 'dark';
    mode.value = false;
} else {
    document.documentElement.classList.remove('dark');
    theme.value = 'light';
    mode.value = true;
}

const changeMode = (event) => {
    if (mode.value) {
        theme.value = 'light';
    }else{
        theme.value = 'dark'; 
    }
    theme.value === 'light' 
        ? document.documentElement.classList.remove('dark')
        : document.documentElement.classList.add('dark');
    localStorage.theme = theme.value;
};

const handleSelect = (key, keyPath) => {
    // Omit!
    console.log(key, keyPath);
};
const handleOpen = (key, keyPath) => {
    // Omit!
    console.log(key, keyPath);
};
const handleClose = (key, keyPath) => {
    // Omit!
    console.log(key, keyPath);
};
const handleCollapse = (event) => {
    isCollapse.value = !isCollapse.value;
};

const checkWindowSize = () => {
    if (window.innerWidth < 1280) {
        if (isCollapse.value === false && innerWidth.value >= 1280) isCollapse.value = true;
    }else{
        if (isCollapse.value === true) isCollapse.value = false;
    }
    innerWidth.value = window.innerWidth;
    innerHeight.value = window.innerHeight;
};

onMounted(() => {
    window.addEventListener('resize', debounce(checkWindowSize, 100));
});

onUnmounted(() => {
    window.removeEventListener('resize', checkWindowSize);
});
</script>

<style>
.flex-grow {
    flex-grow: 1;
}
</style>

<template>
    <div class="common-layout">
        <el-container>
            <el-aside id="side-menu" :width="isCollapse ? '80px' : '210px'" class="duration-300" >
                <el-menu default-active="1" :collapse="isCollapse" :style="'min-height:'+ innerHeight + 'px'" @open="handleOpen" @close="handleClose">
                    <el-menu-item index="0">
                        <div v-if="isCollapse">
                            SC
                        </div>
                        <div v-else>
                            SysCat
                        </div>
                    </el-menu-item>
                    <el-sub-menu index="1">
                        <template #title>
                            <el-icon><IconMenu /></el-icon>
                            <span>System</span>
                        </template>
                        <el-menu-item index="1-1">OS</el-menu-item>
                        <el-menu-item index="1-2">CPU</el-menu-item>
                        <el-menu-item index="1-3">Memory</el-menu-item>
                        <el-menu-item index="1-4">Disk</el-menu-item>
                        <el-menu-item index="1-5">Network</el-menu-item>
                        <el-menu-item index="1-6">User</el-menu-item>
                    </el-sub-menu>
                    <el-sub-menu index="2">
                        <template #title>
                            <el-icon><Odometer /></el-icon>
                            <span>Performance</span>
                        </template>
                        <el-menu-item index="2-1">CPU</el-menu-item>
                        <el-menu-item index="2-2">Memory</el-menu-item>
                        <el-menu-item index="2-3">Disk</el-menu-item>
                        <el-menu-item index="2-4">Network</el-menu-item>
                    </el-sub-menu>
                    <el-menu-item index="3">
                        <el-icon><List /></el-icon>
                        <template #title><span>Process</span></template>
                    </el-menu-item>
                    <el-menu-item index="4">
                        <el-icon><Share /></el-icon>
                        <template #title><span>Network</span></template>
                    </el-menu-item>
                </el-menu>
            </el-aside>
            <el-container>
                <el-header>
                    <el-menu :default-active="activeIndex" mode="horizontal" :ellipsis="false" @select="handleSelect">
                        <el-button type="primary" plain size="large" style="margin-left: 4px; margin-top: 10px" @click="handleCollapse">
                            <el-icon v-if="isCollapse"><Expand /></el-icon>
                            <el-icon v-else><Fold /></el-icon>
                        </el-button>
                        <div class="flex-grow" />
                        <el-menu-item index="0">Overview</el-menu-item>
                        <el-sub-menu index="1">
                            <template #title>Help</template>
                            <el-menu-item index="1-1">About SysCat</el-menu-item>
                            <el-menu-item index="1-2">Check for Update</el-menu-item>
                            <el-menu-item index="1-3">Support</el-menu-item>
                        </el-sub-menu>
                        <el-switch v-model="mode" @click="changeMode" style="margin-left: 24px; margin-top: 12px;" inline-prompt :active-icon="Sunny" :inactive-icon="Moon" />
                    </el-menu>
                </el-header>
                <el-main>
                    <div>
                        <slot />
                    </div>
                </el-main>
            </el-container>
        </el-container>
    </div>
</template>
