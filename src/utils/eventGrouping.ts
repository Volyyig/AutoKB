import type { ScriptEvent } from '../types/script';

export interface EventGroup {
    type: string;
    events: ScriptEvent[];
    expanded: boolean;
    startIndex: number;
}

export function getEventTypeCategory(event: ScriptEvent): string {
    if (event.event_type === 'Delay') return 'Delay';
    if (event.event_type === 'MouseMove') return 'MouseMove';
    if (event.event_type === 'MouseScroll') return 'MouseScroll';
    return event.event_type;
}

export function formatGroupTitle(type: string): string {
    switch (type) {
        case 'Delay': return '等待时间';
        case 'MouseMove': return '鼠标移动';
        case 'MouseScroll': return '鼠标滚轮';
        case 'KeyPress': return '键盘按下';
        case 'KeyRelease': return '键盘弹起';
        case 'MousePress': return '鼠标按键按下';
        case 'MouseRelease': return '鼠标按键弹起';
        default: return type;
    }
}

export function groupEvents(events: ScriptEvent[]): EventGroup[] {
    const groups: EventGroup[] = [];
    if (events.length === 0) return groups;

    let currentGroup: EventGroup | null = null;

    for (let i = 0; i < events.length; i++) {
        const event = events[i];
        let category = getEventTypeCategory(event);

        // Logic to absorb Delay nodes only if they are sandwiched between identical categories
        if (category === 'Delay') {
            // Find the next non-delay event category
            let nextCategory: string | null = null;
            for (let j = i + 1; j < events.length; j++) {
                const nextEventCat = getEventTypeCategory(events[j]);
                if (nextEventCat !== 'Delay') {
                    nextCategory = nextEventCat;
                    break;
                }
            }

            const prevCategory = currentGroup ? currentGroup.type : null;

            // Only absorb if it's a "sandwich" of the same category
            // e.g., MouseMove -> Delay -> MouseMove
            if (nextCategory && prevCategory === nextCategory) {
                category = prevCategory;
            }
            // Otherwise it remains a standalone 'Delay' category
        }

        if (currentGroup && currentGroup.type === category) {
            currentGroup.events.push(event);
        } else {
            if (currentGroup) {
                groups.push(currentGroup);
            }
            currentGroup = {
                type: category,
                events: [event],
                expanded: category !== 'MouseMove',
                startIndex: i
            };
        }
    }

    if (currentGroup) {
        groups.push(currentGroup);
    }

    return groups;
}
