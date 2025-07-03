/**
 * Simple performance monitoring utility for tracking load times
 */
class PerformanceMonitor {
  private measurements: Map<string, number> = new Map();
  
  /**
   * Start timing an operation
   */
  start(operation: string): void {
    this.measurements.set(operation, performance.now());
  }
  
  /**
   * End timing an operation and log the result
   */
  end(operation: string): number {
    const startTime = this.measurements.get(operation);
    if (!startTime) {
      console.warn(`No start time found for operation: ${operation}`);
      return -1;
    }
    
    const duration = performance.now() - startTime;
    this.measurements.delete(operation);
    
    // Log with color based on duration
    if (duration < 100) {
      console.log(`%c⚡ ${operation}: ${duration.toFixed(2)}ms`, 'color: green');
    } else if (duration < 500) {
      console.log(`%c⏱️ ${operation}: ${duration.toFixed(2)}ms`, 'color: orange');
    } else {
      console.log(`%c🐌 ${operation}: ${duration.toFixed(2)}ms`, 'color: red');
    }
    
    return duration;
  }
  
  /**
   * Measure an async operation
   */
  async measure<T>(operation: string, fn: () => Promise<T>): Promise<T> {
    this.start(operation);
    try {
      const result = await fn();
      this.end(operation);
      return result;
    } catch (error) {
      this.end(operation);
      throw error;
    }
  }
}

export const perfMonitor = new PerformanceMonitor();