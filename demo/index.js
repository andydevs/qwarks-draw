/**
 * Demo entry point. Builds a responsive grid of showcase canvases, each bound to its own
 * per-shape canvas class (e.g. CircleCanvas). Every tile resizes its canvas to fill its
 * container and redraws on the window's `resize` event, so its shape stays correctly
 * scaled as the grid reflows.
 */
import { CircleCanvas, LineCanvas, RectCanvas, StarCanvas } from 'jarsdraw-demo'

const SHOWCASE_ITEMS = [
    { selector: '#jarsdraw-canvas-star', Canvas: StarCanvas },
    { selector: '#jarsdraw-canvas-circle', Canvas: CircleCanvas },
    { selector: '#jarsdraw-canvas-line', Canvas: LineCanvas },
    { selector: '#jarsdraw-canvas-rect', Canvas: RectCanvas },
]

for (const { selector, Canvas } of SHOWCASE_ITEMS) {
    // Create showcase canvas
    const showcaseCanvas = new Canvas(selector)

    // Handle resize
    const canvas = document.querySelector(selector)
    const container = canvas.parentElement
    let resizeCanvas = () => {
        canvas.width = container.clientWidth
        canvas.height = container.clientHeight
        showcaseCanvas.redraw()
    }
    resizeCanvas()
    window.addEventListener('resize', () => {
        resizeCanvas()
    })
}
