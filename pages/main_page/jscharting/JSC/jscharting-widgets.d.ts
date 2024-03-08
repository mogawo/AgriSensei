import * as JSC from 'jscharting';
/**
 * Base widget class.
 */
export declare class Base {
    /**
     * Current widget options.
     */
    protected currentOptions: WidgetOptions;
    /**
     * JSCharting chart configuration used to create the widget chart.
     */
    protected currentConfig: JSC.JSCChartConfig;
    /**
     * JSCharting chart reference.
     */
    protected chart: JSC.Chart;
    protected targetDiv: string | any;
    protected targetDivId: string;
    protected styles: any;
    /**
     * Base widget constructor
     * @param targetDiv - Target div id or element.
     * @param options - Widget options.
     * @param callback - Callback function that runs after the chart is generated.
     */
    constructor(targetDiv: string | any, options: WidgetOptions, callback?: (chart: Base) => void);
    /**
     * Updates widget options.
     * @param options - New widget options to update
     * @param updateOptions - Update options such as animation or a callback (then property) when animation is finished.
     */
    options(options: WidgetOptions | string | undefined, updateOptions?: JSC.JSCUpdateOptionsConfig): any;
    /**
     * Redraws the chart. Useful if widget related CSS classes are modified during run-time.
     * @param updateOptions - Update options such as animation or a callback (then property) when animation is finished.
     */
    refresh(updateOptions?: JSC.JSCUpdateOptionsConfig): void;
    /**
     * Refreshes the widget only if css changes.
     */
    refreshResponsive(): void;
    /**
     * Destroys this chart instance.
     */
    destroy(): this;
    /**
     * Processes the widget options into chart options.
     * @param options - Widget options.
     * @param newStyles - If provided new styles will be used instead of detecting.
     * @protected
     */
    protected normalizeOptions(options: WidgetOptions, newStyles?: any): JSC.JSCChartConfig;
    /**
     * Default options for this widget.
     * @protected
     */
    protected getDefaultOptions(): WidgetOptions;
    /**
     * Default chart options for this widget.
     * @protected
     */
    protected getInitConfig(): JSC.JSCChartConfig;
    protected getKind(): string;
    protected needChartRefresh(prevConfig: any, newConfig: any): boolean;
}
/**
 * Base widget options.
 */
export interface WidgetOptions {
    /**
     * Title of the widget shown at the top of the visual.
     */
    title?: string;
    /**
     * The data point label, 'weekdays', 'months', 'quarters', or array of category or date string values.
     */
    label?: string | string[];
    /**
     * Numeric offset of automated labels (like weekdays), or a date that indicates the starting
     * point of automated labels.
     * For example:
     * \{ label: 'months', labelStart: '2/2/2020 \} -> 'Feb', 'Mar', ...
     * Offset by 1:
     * \{ label: 'months', labelStart: 1 \} -> 'Feb', 'Mar', ...
     */
    labelStart?: number;
    /**
     * If set, labels are treated as date values. This string will be used to format the labels. i.e. 'MMM dd'
     */
    timeLabelFormat?: string;
    /**
     * A numeric value or array of values.
     */
    value?: number | number[];
    /**
     * Format string for values. BarColumn only supports a single format value.
     */
    valueFormat?: string | string[];
    /**
     * Text template for values, or array of text templates for each value. The '%value' token is replaced by the value.
     * i.e. 'Count: %value'
     */
    valueText?: string | string[] | Function | Function[];
    /**
     * Formatting culture for all formatting such as numeric, 'weekdays', 'months' label keywords and time labels.
     */
    cultureName?: string;
    /**
     * Color or array of colors for each value.
     */
    color?: string | string[];
    /**
     * Y Axis minimum or array of Y axis minimums. Automatic if not defined.
     */
    min?: number | number[];
    /**
     * Y Axis maximum or array of Y axis maximums. Automatic if not defined.
     */
    max?: number | number[];
    /**
     *  Scales all labels and icons by this amount. For example 1.2 increases label sizes by 20%.
     */
    labelScaling?: number;
    /**
     * Pixel width of the background path the bar fills as the value approaches the maximum.
     * By default the width matches the bar width.
     */
    barBackgroundWidth?: number;
    /**
     * Color of the background path the bar fill as the values approaches the maximum.
     */
    barBackgroundColor?: string;
    /**
     * Pixel width of bar or column data point. Automatic by default.
     */
    barWidth?: number;
    /**
     * Set to false to disable fully rounded bars.
     */
    barRounded?: boolean;
    /**
     * The full chart API can be set through this property directly.
     */
    chartOptions?: JSC.JSCChartConfig;
    [key: string]: any;
}
/**
 * BarColumn widget options.
 */
export interface BarColumnOptions extends WidgetOptions {
    /**
     * If true, the y axis is shown with point tooltips enabled. Otherwise, points are labeled.
     */
    yAxis?: boolean;
    /**
     * True for horizontal bars.
     */
    horizontal?: boolean;
}
/**
 * Linear widget options.
 */
export interface LinearOptions extends WidgetOptions {
    /**
     * Previous value or array of values that correspond to items in the the values property.
     * When specified a green value change appears under the value label when the value increases or a red
     * label when the value decreases.
     */
    prevValue?: number | number[];
    /**
     * Text template for labels, or array of text templates for each label. The '%name' token is replaced by the actual label.
     * i.e. 'Label: %name'
     */
    labelText?: string | string[];
    /**
     * Text template for icons, or array of text templates for each icon. The '%icon' token is used for the actual icon.
     * i.e. '%icon<br>(%value)'
     */
    iconText?: string | string[];
    /**
     * Targets / goals for values. Can be a single target that will apply to each value, or an array
     * of targets applied to each.
     */
    target?: number | number[];
    /**
     * Text template that describes each target. Use a string template to use on each target
     * or an array of strings to customize each target text.
     */
    targetText?: string;
    /**
     * Colors for target values if set. The '%color' token takes on the value color.
     * 'green' - target value is green for all targets
     * ['red', 'green'] - First target is red, second target is green.
     * [ ['red', 'green' ] - All target values are green when not reached and red when reached.
     */
    targetColor?: string | string[];
    /**
     * Icon or array of icons associated with each value. Supports SVG path commands, svg file paths, and raster images file paths.
     * See the icon browser to choose icons
     * https://jscharting.com/examples/chart-features/styling/icons-browser/
     */
    icon?: string | string[];
    /**
     * Position of the icon within the widget. i.e. 'center', 'top', 'top left', 'bottom right', etc.
     * If icon, label, or value positions match then they stack in the icon, value, label order.
     */
    iconPosition?: string;
    /**
     * Position of the value within the widget. i.e. 'center', 'top', 'top left', 'bottom right', etc.
     * If icon, label, or value positions match then they stack in the icon, value, label order.
     */
    valuePosition?: string;
    /**
     * Position of the label within the widget. i.e. 'center', 'top', 'top left', 'bottom right', etc.
     * If icon, label, or value positions match then they stack in the icon, value, label order.
     */
    labelPosition?: string;
}
/**
 * Circular widget options.
 */
export interface CircularOptions extends WidgetOptions {
    /**
     * Previous value or array of values that correspond to items in the the values property.
     * When specified, a green value change appears under the value label when the value increases or a red
     * label when the value decreases.
     */
    prevValue?: number | number[];
    /**
     * Text template for labels, or array of text templates for each label. The '%name' token is replaced by the actual label.
     * i.e. 'Label: %name'
     */
    labelText?: string | string[];
    /**
     * Text template for icons, or array of text templates for each icon. The '%icon' token is used for the actual icon.
     * i.e. '%icon<br>(%value)'
     */
    iconText?: string | string[];
    /**
     * Targets / goals for values. Can be a single target that will apply to each value, or an array
     * of targets applied to each.
     */
    target?: number | number[];
    /**
     * Text template that describes each target. Use a string template to use on each target
     * or an array of strings to customize each target text.
     */
    targetText?: string;
    /**
     * Colors for target values if set. The '%color' token takes on the value color.
     * 'green' - target value is green for all targets
     * ['red', 'green'] - First target is red, second target is green.
     * [ ['red', 'green' ] - All target values are green when not reached and red when reached.
     */
    targetColor?: string | string[];
    /**
     * Icon or array of icons associated with each value. See the icon browser to choose icons
     * https://jscharting.com/examples/chart-features/styling/icons-browser/
     */
    icon?: string | string[];
    /**
     * Position of the icon within the widget. i.e. 'center', 'top', 'top left', 'bottom right', etc.
     * If icon, label, or value positions match then they stack in the icon, value, label order.
     */
    iconPosition?: string;
    /**
     * Position of the value within the widget. i.e. 'center', 'top', 'top left', 'bottom right', etc.
     * If icon, label, or value positions match then they stack in the icon, value, label order.
     */
    valuePosition?: string;
    /**
     * Position of the label within the widget. i.e. 'center', 'top', 'top left', 'bottom right', etc.
     * If icon, label, or value positions match then they stack in the icon, value, label order.
     */
    labelPosition?: string;
    /**
     * Angle sweep for all circular widget values or an array of individual sweeps for each value.
     */
    sweep?: number | number[];
}
/**
 * The BarColumn widget creates a small version of a bar or column chart.
 */
export declare class BarColumn extends Base {
    /**
     * Current widget options.
     */
    protected currentOptions: BarColumnOptions;
    /**
     * Create a bar column widget instance.
     * @param targetDiv - Target div for the widget visual.
     * @param options - BarColumn widget options.
     * @param callback - Callback function that runs after the chart is generated. This widget is passed as an argument.
     */
    constructor(targetDiv: string | any, options: BarColumnOptions, callback?: (widget: BarColumn) => void);
    /**
     * Updates circular widget options.
     * @param options - New widget options to update
     * @param updateOptions - Update options such as animation or a callback (then property) when animation is finished.
     */
    options(options: BarColumnOptions | string | undefined, updateOptions?: JSC.JSCUpdateOptionsConfig): any;
    /**
     * Processes the widget options into chart options.
     * @param options - Widget options.
     * @param newStyles - Updated CSS styles to use if available. If not provided styles are checked.
     * @protected
     */
    protected normalizeOptions(options: BarColumnOptions, newStyles?: any): JSC.JSCChartConfig;
    /**
     * Default options for this widget.
     * @protected
     */
    protected getDefaultOptions(): BarColumnOptions;
    /**
     * Default chart options for this widget.
     * @protected
     */
    protected getInitConfig(): JSC.JSCChartConfig;
    protected getKind(): string;
}
/**
 * Circular widget class.
 */
export declare class Circular extends Base {
    /**
     * Current widget options.
     */
    protected currentOptions: CircularOptions;
    /**
     * Initialize a JSC widget visual in the provided targetDiv.
     * @param targetDiv - Target div id or element.
     * @param options - Circular widget options.
     * @param callback - Callback function that runs after the chart is generated. This widget is passed as an argument.
     */
    constructor(targetDiv: string | any, options: CircularOptions, callback?: (widget: Circular) => void);
    /**
     * Updates circular widget options.
     * @param options - New widget options to update
     * @param updateOptions - Update options such as animation or a callback (then property) when animation is finished.
     */
    options(options: CircularOptions | string | undefined, updateOptions?: JSC.JSCUpdateOptionsConfig): any;
    /**
     * Processes the widget options into chart options.
     * @param options - Widget options.
     * @param newStyles - Updated CSS styles to use if available. If not provided styles are checked.
     * @protected
     */
    protected normalizeOptions(options: CircularOptions, newStyles?: any): JSC.JSCChartConfig;
    /**
     * Default options for this widget.
     * @protected
     */
    protected getDefaultOptions(): CircularOptions;
    /**
     * Default chart options for this widget.
     * @protected
     */
    protected getInitConfig(): JSC.JSCChartConfig;
    protected getKind(): string;
}
/**
 * Linear widget class.
 */
export declare class Linear extends Base {
    /**
     * Current widget options.
     */
    protected currentOptions: LinearOptions;
    /**
     * Initialize a JSC widget visual in the provided targetDiv.
     * @param targetDiv - Target div for the widget visual.
     * @param options - Linear widget options.
     * @param callback - Callback function that runs after the chart is generated. This widget is passed as an argument.
     */
    constructor(targetDiv: string | any, options: LinearOptions, callback?: (widget: Linear) => void);
    /**
     * Updates linear widget options.
     * @param options - New widget options to update
     * @param updateOptions - Update options such as animation or a callback (then property) when animation is finished.
     */
    options(options: LinearOptions | string | undefined, updateOptions?: JSC.JSCUpdateOptionsConfig): any;
    /**
     * Processes the widget options into chart options.
     * @param options - Widget options.
     * @param newStyles - Updated CSS styles to use if available. If not provided styles are checked.
     * @protected
     */
    protected normalizeOptions(options: LinearOptions, newStyles?: any): JSC.JSCChartConfig;
    /**
     * Default options for this widget.
     * @protected
     */
    protected getDefaultOptions(): LinearOptions;
    /**
     * Default chart options for this widget.
     * @protected
     */
    protected getInitConfig(): JSC.JSCChartConfig;
    protected getKind(): string;
}
/**
 * Used to define any global JSCharting options such as debug or baseUrl for JSCharting modules.
 * @param options - Global options used by all JSCharting charts instantiated after these defaults are set.
 */
export declare function jscDefaults(options?: JSC.JSCChartConfig | undefined): JSC.JSCChartConfig;
