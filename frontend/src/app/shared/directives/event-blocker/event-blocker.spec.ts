import { EventBlocker } from './event-blocker';

describe('EventBlocker', () => {
  it('should create an instance', () => {
    const directive = new EventBlocker();
    expect(directive).toBeTruthy();
  });
});
