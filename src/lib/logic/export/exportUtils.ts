/**
 * Descarga bytes como archivo
 */
export function downloadBytes(bytes: number[], filename: string) {
    const blob = new Blob([new Uint8Array(bytes)]);
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
}

/**
 * Abre archivo PDF en nueva pestaña (preview)
 */
export function previewPDF(bytes: number[]) {
    try {
        const blob = new Blob([new Uint8Array(bytes)], { type: 'application/pdf' });
        const url = URL.createObjectURL(blob);

        // Intentar abrir en nueva pestaña
        const newWindow = window.open(url, '_blank');

        // Verificar si fue bloqueado por popup blocker
        if (!newWindow || newWindow.closed || typeof newWindow.closed === 'undefined') {
            // Alternativa: crear link de descarga temporal
            const link = document.createElement('a');
            link.href = url;
            link.download = `preview-${Date.now()}.pdf`;
            link.target = '_blank';
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);

            throw new Error('El navegador bloqueó la ventana emergente. Descargando PDF en su lugar.');
        }

        // Limpiar URL después de un tiempo
        setTimeout(() => URL.revokeObjectURL(url), 60000);
    } catch (error) {
        throw error;
    }
}

/**
 * Convierte un valor de margen a Centímetros según la unidad dada.
 */
export function marginToCm(value: number, unit: string): number {
    switch (unit) {
        case "mm":
            return value / 10;
        case "in":
            return value * 2.54;
        case "pt":
            return value / 28.35; // 1cm = 28.35pt
        default:
            return value; // cm
    }
}
