console.log("Test App - Iced Plugin Demo");

document.getElementById('openIcedWindow').addEventListener('click', async () => {
    try {
        await window.__TAURI__.core.invoke('create_iced_window');
        console.log('Iced window created successfully');
    } catch (error) {
        console.error('Failed to create iced window:', error);
    }
});
