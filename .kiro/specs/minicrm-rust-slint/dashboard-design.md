# MiniCRM 仪表盘界面设计

## 仪表盘整体布局

```
┌─────────────────────────────────────────────────────────────┐
│                        数据仪表盘                           │
├─────────────────────────────────────────────────────────────┤
│  关键指标卡片区域 (KPI Cards)                               │
│ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐             │
│ │客户数量 │ │待办任务 │ │ 报价   │ │ 应收   │             │
│ │  156   │ │   8    │ │  12个  │ │ ¥50.2K │             │
│ │  个    │ │  项    │ │ 待确认  │ │ 待收回  │             │
│ └─────────┘ └─────────┘ └─────────┘ └─────────┘             │
├─────────────────────────────────────────────────────────────┤
│  主要业务区域 (Primary Business Area)                       │
│ ┌─────────────────────┐ ┌─────────────────────────────────┐ │
│ │     待办任务列表    │ │         售后工单列表            │ │
│ │                     │ │                                 │ │
│ │  � 优先月级任务      │ │   🔧 待处理工单                │ │
│ │                     │ │                                 │ │
│ └─────────────────────┘ └─────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  图表分析区域 (Charts Analysis Area)                        │
│ ┌─────────────────────┐ ┌─────────────────────────────────┐ │
│ │   待办任务分类图    │ │      售后问题分类图             │ │
│ │                     │ │                                 │ │
│ │  📊 紧急/重要/普通   │ │   🔧 开胶/崩边/起皮/起皱/变型 │ │
│ │                     │ │                                 │ │
│ └─────────────────────┘ └─────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  快速操作和通知区域                                         │
│ ┌─────────────────────┐ ┌─────────────────────────────────┐ │
│ │    快速操作         │ │         最新通知                │ │
│ │ [新增客户]          │ │  • 客户A的合同即将到期          │ │
│ │ [创建报价]          │ │  • 供应商B提交了新报价          │ │
│ │ [记录收款]          │ │  • 3个售后工单待处理            │ │
│ │ [查看报表]          │ │  • 本月销售目标完成80%          │ │
│ └─────────────────────┘ └─────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Slint仪表盘实现

```slint
// src/presentation/ui/pages/dashboard.slint
import { Button, VerticalBox, HorizontalBox, ScrollView, Text } from "std-widgets.slint";
import { AppTheme } from "../themes/default_theme.slint";
import { KpiCard } from "../components/kpi_card.slint";
import { LineChart } from "../components/line_chart.slint";
import { PieChart } from "../components/pie_chart.slint";
import { BarChart } from "../components/bar_chart.slint";

/// 仪表盘页面组件
/// 
/// 显示系统的关键业务指标、图表分析和快速操作入口。
/// 为用户提供业务概览和数据洞察。
/// 
/// # 功能特性
/// 
/// 1. **关键指标展示**: 客户数量、财务数据、任务统计等KPI
/// 2. **趋势分析**: 客户增长、互动频率等趋势图表
/// 3. **分布分析**: 客户类型、账款状态等分布图表
/// 4. **快速操作**: 常用功能的快捷入口
/// 5. **实时通知**: 重要事件和提醒信息
/// 
/// # 数据更新
/// 
/// - 自动刷新：每5分钟自动更新数据
/// - 手动刷新：用户可以手动触发数据刷新
/// - 实时更新：关键操作后立即更新相关数据
/// 
/// # 响应式设计
/// 
/// - 支持不同屏幕尺寸的自适应布局
/// - 图表组件支持动态调整大小
/// - 移动端友好的交互设计
export component DashboardPage {
    // 输入属性 - 仪表盘数据
    in property <DashboardData> dashboard-data: {
        kpi: {
            total-customers: 0,
            pending-tasks: 0,
            pending-quotes: 0,
            accounts-receivable: 0.0,
            customer-growth-rate: 0.0,
            quote-growth-rate: 0.0,
            revenue-growth-rate: 0.0,
        },
        charts: {
            task-classification: [],
            service-classification: [],
            interaction-frequency: [],
            receivables-status: [],
        },
        notifications: [],
        quick-actions: true,
    };
    
    // 状态属性
    in property <bool> loading: false;
    in property <string> last-updated: "";
    
    // 回调函数
    callback refresh-data();
    callback navigate-to(string);
    callback quick-action(string);
    callback notification-clicked(int);
    
    // 主滚动容器
    ScrollView {
        VerticalBox {
            spacing: AppTheme.spacing-lg;
            padding: AppTheme.spacing-lg;
            
            // 页面标题和刷新按钮
            header := HorizontalBox {
                alignment: space-between;
                
                title-section := VerticalBox {
                    alignment: start;
                    
                    Text {
                        text: "数据仪表盘";
                        font-size: AppTheme.font-size-title;
                        font-weight: 700;
                        color: AppTheme.text-primary;
                    }
                    
                    Text {
                        text: "最后更新: " + last-updated;
                        font-size: AppTheme.font-size-small;
                        color: AppTheme.text-secondary;
                    }
                }
                
                refresh-section := HorizontalBox {
                    spacing: AppTheme.spacing-sm;
                    
                    Button {
                        text: loading ? "刷新中..." : "🔄 刷新数据";
                        enabled: !loading;
                        
                        clicked => {
                            refresh-data();
                        }
                    }
                    
                    Button {
                        text: "⚙️ 设置";
                        
                        clicked => {
                            navigate-to("settings");
                        }
                    }
                }
            }
            
            // 关键指标卡片区域
            kpi-section := VerticalBox {
                spacing: AppTheme.spacing-md;
                
                Text {
                    text: "关键业务指标";
                    font-size: AppTheme.font-size-heading;
                    font-weight: 600;
                    color: AppTheme.text-primary;
                }
                
                kpi-cards := HorizontalBox {
                    spacing: AppTheme.spacing-md;
                    
                    // 客户数量卡片
                    KpiCard {
                        title: "客户数量";
                        value: dashboard-data.kpi.total-customers;
                        unit: "个";
                        trend: dashboard-data.kpi.customer-growth-rate;
                        icon: "👥";
                        color: AppTheme.primary-color;
                        
                        clicked => {
                            navigate-to("customers");
                        }
                    }
                    
                    // 待办任务卡片
                    KpiCard {
                        title: "待办任务";
                        value: dashboard-data.kpi.pending-tasks;
                        unit: "项";
                        trend: 0.0;
                        icon: "📋";
                        color: AppTheme.warning-color;
                        urgent: dashboard-data.kpi.pending-tasks > 10;
                        
                        clicked => {
                            navigate-to("tasks");
                        }
                    }
                    
                    // 报价卡片
                    KpiCard {
                        title: "报价";
                        value: dashboard-data.kpi.pending-quotes;
                        unit: "个";
                        trend: dashboard-data.kpi.quote-growth-rate;
                        icon: "�";
                        color: AppTheme.info-color;
                        
                        clicked => {
                            navigate-to("quotes");
                        }
                    }
                    
                    // 应收账款卡片
                    KpiCard {
                        title: "应收";
                        value: dashboard-data.kpi.accounts-receivable;
                        unit: "万元";
                        trend: dashboard-data.kpi.revenue-growth-rate;
                        icon: "💰";
                        color: AppTheme.success-color;
                        is-currency: true;
                        
                        clicked => {
                            navigate-to("finance/receivables");
                        }
                    }
                }
            }
            
            // 主要业务区域
            primary-business := VerticalBox {
                spacing: AppTheme.spacing-md;
                
                Text {
                    text: "重要业务事项";
                    font-size: AppTheme.font-size-heading;
                    font-weight: 600;
                    color: AppTheme.text-primary;
                }
                
                business-row := HorizontalBox {
                    spacing: AppTheme.spacing-md;
                    
                    // 待办任务列表
                    todo-list := Rectangle {
                        background: AppTheme.card-color;
                        border-radius: AppTheme.border-radius-md;
                        border-width: 1px;
                        border-color: AppTheme.border-color;
                        
                        VerticalBox {
                            padding: AppTheme.spacing-md;
                            spacing: AppTheme.spacing-sm;
                            
                            // 任务列表标题
                            todo-header := HorizontalBox {
                                alignment: space-between;
                                
                                Text {
                                    text: "待办任务";
                                    font-size: AppTheme.font-size-large;
                                    font-weight: 600;
                                    color: AppTheme.text-primary;
                                }
                                
                                HorizontalBox {
                                    spacing: AppTheme.spacing-xs;
                                    
                                    Text {
                                        text: dashboard-data.kpi.pending-tasks;
                                        font-size: AppTheme.font-size-small;
                                        color: AppTheme.warning-color;
                                        font-weight: 600;
                                    }
                                    
                                    Text {
                                        text: "项待办";
                                        font-size: AppTheme.font-size-small;
                                        color: AppTheme.text-secondary;
                                    }
                                }
                            }
                            
                            // 任务列表内容
                            if dashboard-data.tasks.length > 0: ScrollView {
                                height: 200px;
                                
                                VerticalBox {
                                    spacing: AppTheme.spacing-xs;
                                    
                                    for task[index] in dashboard-data.tasks: Rectangle {
                                        background: task.urgent ? AppTheme.error-color.with-alpha(0.1) : 
                                                   task.priority == "high" ? AppTheme.warning-color.with-alpha(0.1) : 
                                                   transparent;
                                        border-radius: AppTheme.border-radius-sm;
                                        border-left: 3px solid (task.urgent ? AppTheme.error-color : 
                                                                task.priority == "high" ? AppTheme.warning-color : 
                                                                AppTheme.info-color);
                                        
                                        HorizontalBox {
                                            padding: AppTheme.spacing-sm;
                                            spacing: AppTheme.spacing-sm;
                                            alignment: space-between;
                                            
                                            // 任务内容
                                            VerticalBox {
                                                alignment: start;
                                                
                                                Text {
                                                    text: task.title;
                                                    font-size: AppTheme.font-size-normal;
                                                    color: AppTheme.text-primary;
                                                    font-weight: 500;
                                                    overflow: elide;
                                                }
                                                
                                                HorizontalBox {
                                                    spacing: AppTheme.spacing-sm;
                                                    
                                                    Text {
                                                        text: task.customer-name;
                                                        font-size: AppTheme.font-size-small;
                                                        color: AppTheme.text-secondary;
                                                    }
                                                    
                                                    Text {
                                                        text: "•";
                                                        font-size: AppTheme.font-size-small;
                                                        color: AppTheme.text-disabled;
                                                    }
                                                    
                                                    Text {
                                                        text: task.due-date;
                                                        font-size: AppTheme.font-size-small;
                                                        color: task.overdue ? AppTheme.error-color : AppTheme.text-secondary;
                                                    }
                                                }
                                            }
                                            
                                            // 优先级标签
                                            Rectangle {
                                                width: 60px;
                                                height: 20px;
                                                border-radius: 10px;
                                                background: task.urgent ? AppTheme.error-color : 
                                                           task.priority == "high" ? AppTheme.warning-color : 
                                                           AppTheme.info-color;
                                                
                                                Text {
                                                    text: task.urgent ? "紧急" : 
                                                         task.priority == "high" ? "重要" : "普通";
                                                    font-size: 10px;
                                                    color: white;
                                                    font-weight: 500;
                                                    horizontal-alignment: center;
                                                    vertical-alignment: center;
                                                }
                                            }
                                        }
                                        
                                        TouchArea {
                                            clicked => {
                                                navigate-to("tasks/" + task.id);
                                            }
                                        }
                                    }
                                }
                            }
                            
                            if dashboard-data.tasks.length == 0: VerticalBox {
                                alignment: center;
                                height: 150px;
                                
                                Text {
                                    text: "✅";
                                    font-size: 32px;
                                }
                                
                                Text {
                                    text: "暂无待办任务";
                                    font-size: AppTheme.font-size-normal;
                                    color: AppTheme.text-secondary;
                                }
                            }
                            
                            // 查看全部按钮
                            Button {
                                text: "查看全部任务";
                                width: 100%;
                                
                                clicked => {
                                    navigate-to("tasks");
                                }
                            }
                        }
                    }
                    
                    // 售后工单列表
                    service-tickets := Rectangle {
                        background: AppTheme.card-color;
                        border-radius: AppTheme.border-radius-md;
                        border-width: 1px;
                        border-color: AppTheme.border-color;
                        
                        VerticalBox {
                            padding: AppTheme.spacing-md;
                            spacing: AppTheme.spacing-sm;
                            
                            // 工单列表标题
                            tickets-header := HorizontalBox {
                                alignment: space-between;
                                
                                Text {
                                    text: "售后工单";
                                    font-size: AppTheme.font-size-large;
                                    font-weight: 600;
                                    color: AppTheme.text-primary;
                                }
                                
                                HorizontalBox {
                                    spacing: AppTheme.spacing-xs;
                                    
                                    Text {
                                        text: dashboard-data.service-tickets.length;
                                        font-size: AppTheme.font-size-small;
                                        color: AppTheme.error-color;
                                        font-weight: 600;
                                    }
                                    
                                    Text {
                                        text: "个工单";
                                        font-size: AppTheme.font-size-small;
                                        color: AppTheme.text-secondary;
                                    }
                                }
                            }
                            
                            // 工单列表内容
                            if dashboard-data.service-tickets.length > 0: ScrollView {
                                height: 200px;
                                
                                VerticalBox {
                                    spacing: AppTheme.spacing-xs;
                                    
                                    for ticket[index] in dashboard-data.service-tickets: Rectangle {
                                        background: ticket.status == "urgent" ? AppTheme.error-color.with-alpha(0.1) : 
                                                   ticket.status == "pending" ? AppTheme.warning-color.with-alpha(0.1) : 
                                                   AppTheme.success-color.with-alpha(0.1);
                                        border-radius: AppTheme.border-radius-sm;
                                        border-left: 3px solid (ticket.status == "urgent" ? AppTheme.error-color : 
                                                                ticket.status == "pending" ? AppTheme.warning-color : 
                                                                AppTheme.success-color);
                                        
                                        HorizontalBox {
                                            padding: AppTheme.spacing-sm;
                                            spacing: AppTheme.spacing-sm;
                                            alignment: space-between;
                                            
                                            // 工单内容
                                            VerticalBox {
                                                alignment: start;
                                                
                                                Text {
                                                    text: ticket.title;
                                                    font-size: AppTheme.font-size-normal;
                                                    color: AppTheme.text-primary;
                                                    font-weight: 500;
                                                    overflow: elide;
                                                }
                                                
                                                HorizontalBox {
                                                    spacing: AppTheme.spacing-sm;
                                                    
                                                    Text {
                                                        text: ticket.customer-name;
                                                        font-size: AppTheme.font-size-small;
                                                        color: AppTheme.text-secondary;
                                                    }
                                                    
                                                    Text {
                                                        text: "•";
                                                        font-size: AppTheme.font-size-small;
                                                        color: AppTheme.text-disabled;
                                                    }
                                                    
                                                    Text {
                                                        text: ticket.product-name;
                                                        font-size: AppTheme.font-size-small;
                                                        color: AppTheme.text-secondary;
                                                    }
                                                    
                                                    Text {
                                                        text: "•";
                                                        font-size: AppTheme.font-size-small;
                                                        color: AppTheme.text-disabled;
                                                    }
                                                    
                                                    Text {
                                                        text: ticket.created-date;
                                                        font-size: AppTheme.font-size-small;
                                                        color: AppTheme.text-secondary;
                                                    }
                                                }
                                            }
                                            
                                            // 状态标签
                                            Rectangle {
                                                width: 60px;
                                                height: 20px;
                                                border-radius: 10px;
                                                background: ticket.status == "urgent" ? AppTheme.error-color : 
                                                           ticket.status == "pending" ? AppTheme.warning-color : 
                                                           AppTheme.success-color;
                                                
                                                Text {
                                                    text: ticket.status == "urgent" ? "紧急" : 
                                                         ticket.status == "pending" ? "待处理" : "已处理";
                                                    font-size: 10px;
                                                    color: white;
                                                    font-weight: 500;
                                                    horizontal-alignment: center;
                                                    vertical-alignment: center;
                                                }
                                            }
                                        }
                                        
                                        TouchArea {
                                            clicked => {
                                                navigate-to("service-tickets/" + ticket.id);
                                            }
                                        }
                                    }
                                }
                            }
                            
                            if dashboard-data.service-tickets.length == 0: VerticalBox {
                                alignment: center;
                                height: 150px;
                                
                                Text {
                                    text: "🔧";
                                    font-size: 32px;
                                }
                                
                                Text {
                                    text: "暂无售后工单";
                                    font-size: AppTheme.font-size-normal;
                                    color: AppTheme.text-secondary;
                                }
                            }
                            
                            // 查看全部按钮
                            Button {
                                text: "查看全部工单";
                                width: 100%;
                                
                                clicked => {
                                    navigate-to("service-tickets");
                                }
                            }
                        }
                    }
                }
            }
            
            // 图表分析区域
            charts-analysis := VerticalBox {
                spacing: AppTheme.spacing-md;
                
                Text {
                    text: "数据分析图表";
                    font-size: AppTheme.font-size-heading;
                    font-weight: 600;
                    color: AppTheme.text-primary;
                }
                
                charts-row := HorizontalBox {
                    spacing: AppTheme.spacing-md;
                    
                    // 待办任务分类图
                    task-classification-chart := Rectangle {
                        background: AppTheme.card-color;
                        border-radius: AppTheme.border-radius-md;
                        border-width: 1px;
                        border-color: AppTheme.border-color;
                        
                        VerticalBox {
                            padding: AppTheme.spacing-md;
                            spacing: AppTheme.spacing-sm;
                            
                            // 图表标题
                            chart-header := HorizontalBox {
                                alignment: space-between;
                                
                                Text {
                                    text: "待办任务分类";
                                    font-size: AppTheme.font-size-large;
                                    font-weight: 600;
                                    color: AppTheme.text-primary;
                                }
                                
                                Text {
                                    text: "按优先级";
                                    font-size: AppTheme.font-size-small;
                                    color: AppTheme.text-secondary;
                                }
                            }
                            
                            // 柱状图组件
                            BarChart {
                                width: 100%;
                                height: 200px;
                                data: dashboard-data.charts.task-classification;
                                x-axis-label: "优先级";
                                y-axis-label: "任务数量";
                                bar-colors: [
                                    AppTheme.error-color,    // 紧急 - 红色
                                    AppTheme.warning-color,  // 重要 - 橙色
                                    AppTheme.info-color,     // 普通 - 蓝色
                                ];
                                show-grid: true;
                                show-values: true;
                                
                                bar-clicked(index, label, value) => {
                                    // 处理柱状图点击事件
                                    navigate-to("tasks?priority=" + label);
                                }
                            }
                        }
                    }
                    
                    // 售后问题分类图
                    service-classification-chart := Rectangle {
                        background: AppTheme.card-color;
                        border-radius: AppTheme.border-radius-md;
                        border-width: 1px;
                        border-color: AppTheme.border-color;
                        
                        VerticalBox {
                            padding: AppTheme.spacing-md;
                            spacing: AppTheme.spacing-sm;
                            
                            // 图表标题
                            Text {
                                text: "售后问题分类";
                                font-size: AppTheme.font-size-large;
                                font-weight: 600;
                                color: AppTheme.text-primary;
                            }
                            
                            // 饼图组件
                            PieChart {
                                width: 100%;
                                height: 200px;
                                data: dashboard-data.charts.service-classification;
                                show-legend: true;
                                show-labels: true;
                                colors: [
                                    AppTheme.error-color,    // 开胶 - 红色
                                    AppTheme.warning-color,  // 崩边 - 橙色
                                    AppTheme.info-color,     // 起皮 - 蓝色
                                    AppTheme.success-color,  // 起皱 - 绿色
                                    AppTheme.primary-color,  // 变型 - 主色
                                ];
                                
                                segment-clicked(index, label, value) => {
                                    // 处理饼图段点击事件
                                    navigate-to("service-tickets?category=" + label);
                                }
                            }
                        }
                    }
                }
            }
            
            // 快速操作和通知区域
            bottom-section := HorizontalBox {
                spacing: AppTheme.spacing-md;
                alignment: stretch;
                
                // 快速操作面板
                quick-actions-panel := Rectangle {
                    background: AppTheme.card-color;
                    border-radius: AppTheme.border-radius-md;
                    border-width: 1px;
                    border-color: AppTheme.border-color;
                    width: 50%;
                    
                    VerticalBox {
                        padding: AppTheme.spacing-md;
                        spacing: AppTheme.spacing-md;
                        
                        Text {
                            text: "快速操作";
                            font-size: AppTheme.font-size-large;
                            font-weight: 600;
                            color: AppTheme.text-primary;
                        }
                        
                        actions-grid := VerticalBox {
                            spacing: AppTheme.spacing-sm;
                            
                            actions-row-1 := HorizontalBox {
                                spacing: AppTheme.spacing-sm;
                                
                                Button {
                                    text: "➕ 新增客户";
                                    primary: true;
                                    
                                    clicked => {
                                        quick-action("new-customer");
                                    }
                                }
                                
                                Button {
                                    text: "🏭 新增供应商";
                                    
                                    clicked => {
                                        quick-action("new-supplier");
                                    }
                                }
                            }
                            
                            actions-row-2 := HorizontalBox {
                                spacing: AppTheme.spacing-sm;
                                
                                Button {
                                    text: "📋 创建报价";
                                    
                                    clicked => {
                                        quick-action("new-quote");
                                    }
                                }
                                
                                Button {
                                    text: "💰 记录收款";
                                    
                                    clicked => {
                                        quick-action("record-payment");
                                    }
                                }
                            }
                            
                            actions-row-3 := HorizontalBox {
                                spacing: AppTheme.spacing-sm;
                                
                                Button {
                                    text: "📊 查看报表";
                                    
                                    clicked => {
                                        quick-action("view-reports");
                                    }
                                }
                                
                                Button {
                                    text: "📄 新建合同";
                                    
                                    clicked => {
                                        quick-action("new-contract");
                                    }
                                }
                            }
                        }
                    }
                }
                
                // 通知面板
                notifications-panel := Rectangle {
                    background: AppTheme.card-color;
                    border-radius: AppTheme.border-radius-md;
                    border-width: 1px;
                    border-color: AppTheme.border-color;
                    width: 50%;
                    
                    VerticalBox {
                        padding: AppTheme.spacing-md;
                        spacing: AppTheme.spacing-md;
                        
                        notifications-header := HorizontalBox {
                            alignment: space-between;
                            
                            Text {
                                text: "最新通知";
                                font-size: AppTheme.font-size-large;
                                font-weight: 600;
                                color: AppTheme.text-primary;
                            }
                            
                            Text {
                                text: dashboard-data.notifications.length + " 条";
                                font-size: AppTheme.font-size-small;
                                color: AppTheme.text-secondary;
                            }
                        }
                        
                        if dashboard-data.notifications.length > 0: ScrollView {
                            height: 200px;
                            
                            VerticalBox {
                                spacing: AppTheme.spacing-xs;
                                
                                for notification[index] in dashboard-data.notifications: Rectangle {
                                    background: notification.urgent ? AppTheme.error-color.with-alpha(0.1) : transparent;
                                    border-radius: AppTheme.border-radius-sm;
                                    
                                    HorizontalBox {
                                        padding: AppTheme.spacing-sm;
                                        spacing: AppTheme.spacing-sm;
                                        
                                        // 通知图标
                                        Text {
                                            text: notification.urgent ? "🔴" : 
                                                 notification.type == "contract" ? "📄" :
                                                 notification.type == "payment" ? "💰" :
                                                 notification.type == "task" ? "📋" : "ℹ️";
                                            font-size: AppTheme.font-size-normal;
                                        }
                                        
                                        // 通知内容
                                        VerticalBox {
                                            alignment: start;
                                            
                                            Text {
                                                text: notification.title;
                                                font-size: AppTheme.font-size-normal;
                                                color: AppTheme.text-primary;
                                                font-weight: 500;
                                                overflow: elide;
                                            }
                                            
                                            Text {
                                                text: notification.message;
                                                font-size: AppTheme.font-size-small;
                                                color: AppTheme.text-secondary;
                                                overflow: elide;
                                            }
                                            
                                            Text {
                                                text: notification.time;
                                                font-size: AppTheme.font-size-small;
                                                color: AppTheme.text-disabled;
                                            }
                                        }
                                    }
                                    
                                    TouchArea {
                                        clicked => {
                                            notification-clicked(index);
                                        }
                                    }
                                }
                            }
                        }
                        
                        if dashboard-data.notifications.length == 0: VerticalBox {
                            alignment: center;
                            height: 100px;
                            
                            Text {
                                text: "🔔";
                                font-size: 32px;
                            }
                            
                            Text {
                                text: "暂无新通知";
                                font-size: AppTheme.font-size-normal;
                                color: AppTheme.text-secondary;
                            }
                        }
                    }
                }
            }
        }
    }
}

// 仪表盘数据结构定义
export struct DashboardData {
    kpi: KpiData,
    charts: ChartData,
    notifications: [NotificationData],
    quick-actions: bool,
}

export struct KpiData {
    total-customers: int,
    pending-tasks: int,
    pending-quotes: int,
    accounts-receivable: float,
    customer-growth-rate: float,
    quote-growth-rate: float,
    revenue-growth-rate: float,
}

export struct ChartData {
    task-classification: [BarData],
    service-classification: [PieSegment],
    interaction-frequency: [BarData],
    receivables-status: [BarData],
}

export struct ChartPoint {
    x: string,
    y: float,
}

export struct PieSegment {
    label: string,
    value: float,
    color: color,
}

export struct BarData {
    label: string,
    value: float,
}

export struct NotificationData {
    id: int,
    type: string,
    title: string,
    message: string,
    time: string,
    urgent: bool,
}

export struct CustomFieldData {
    id: int,
    field-type: string,  // "service-issue" | "service-method" | "task-priority" 等
    name: string,
    display-name: string,
    color: color,
    sort-order: int,
    is-active: bool,
}
```

## 仪表盘设计更新说明

### 关键指标卡片区域调整
- **客户数量**: 显示总客户数量，保持原有功能
- **待办任务**: 显示待处理任务数量，支持紧急任务提醒
- **报价**: 显示待确认的报价数量，新增功能
- **应收**: 显示应收账款金额，简化显示名称

### 图表分析区域调整
- **待办任务分类图**: 替代客户增长趋势图，按优先级（紧急/重要/普通）分类显示任务
- **售后问题分类图**: 替代客户类型分布图，按板材行业问题类型（开胶/崩边/起皮/起皱/变型等）分类显示售后工单

### 自定义字段支持
- **售后问题分类**: 支持用户自定义问题类型，默认包含开胶、崩边、起皮、起皱、变型等板材行业常见问题
- **售后处理方式**: 支持用户自定义处理方式，默认包含赔板、赔款、退货等常见处理方式
- **动态图表更新**: 图表会根据用户的自定义字段配置自动调整显示内容和颜色
- **数据迁移保护**: 修改自定义字段时提供数据迁移和备份机制

### 技术实现特点

1. **信息层次清晰** - KPI卡片 → 业务列表 → 分类图表 → 快速操作
2. **交互性强** - 所有图表和卡片都支持点击跳转到详细页面
3. **实时更新** - 支持自动和手动数据刷新机制
4. **响应式设计** - 适应不同屏幕尺寸的自适应布局
5. **视觉吸引** - 丰富的图表类型和统一的颜色搭配
6. **操作便捷** - 集成快速操作面板和通知中心
7. **业务导向** - 重点关注待办任务和售后服务管理