import type { ScriptEvent } from '../types/script';

export interface EventGroup {
    type: string;
    events: ScriptEvent[];
    expanded: boolean;
    startIndex: number;
}

export function getEventTypeCategory(event: ScriptEvent): string {
    if (event.event_type === 'MouseMove') return 'MouseMove';
    if (event.event_type === 'MouseScroll') return 'MouseScroll';
    return event.event_type;
}

export function formatGroupTitle(type: string): string {
    switch (type) {
        case 'MouseMove': return 'Mouse Movements';
        case 'MouseScroll': return 'Scroll Events';
        case 'KeyPress': return 'Key Presses';
        case 'KeyRelease': return 'Key Releases';
        case 'MousePress': return 'Mouse Clicks (Down)';
        case 'MouseRelease': return 'Mouse Clicks (Up)';
        default: return type;
    }
}

export function groupEvents(events: ScriptEvent[]): EventGroup[] {
    const groups: EventGroup[] = [];
    if (events.length === 0) return groups;

    let currentGroup: EventGroup | null = null;

    events.forEach((event, index) => {
        const type = getEventTypeCategory(event);

        if (currentGroup && currentGroup.type === type) {
            currentGroup.events.push(event);
        } else {
            if (currentGroup) {
                groups.push(currentGroup);
            }
            currentGroup = {
                type,
                events: [event],
                expanded: false,
                startIndex: index
            };
        }
    });

    if (currentGroup) {
        groups.push(currentGroup);
    }

    return groups;
}
