## Slint UI代码规范

### 1. Slint组件结构规范

```slint
// src/presentation/ui/components/customer_list.slint
import { Button, ListView, HorizontalBox, VerticalBox, Text } from "std-widgets.slint";

/// 客户列表组件
/// 
/// 显示客户列表，支持搜索、筛选、分页等功能。
/// 
/// # 属性说明
/// 
/// - `customers`: 客户数据数组
/// - `loading`: 是否正在加载数据
/// - `selected-customer`: 当前选中的客户
/// 
/// # 回调函数
/// 
/// - `customer-selected`: 客户被选中时触发
/// - `customer-edited`: 客户编辑按钮被点击时触发
/// - `customer-deleted`: 客户删除按钮被点击时触发
/// - `refresh-requested`: 刷新按钮被点击时触发
/// 
/// # 使用示例
/// 
/// ```slint
/// CustomerList {
///     customers: customer-data;
///     loading: is-loading;
///     customer-selected(customer) => {
///         // 处理客户选择逻辑
///     }
/// }
/// ```
export component CustomerList {
    // 输入属性 - 使用 in 关键字标记只读属性
    in property <[CustomerData]> customers: [];
    in property <bool> loading: false;
    in property <string> search-keyword: "";
    in property <CustomerLevel> filter-level: CustomerLevel.all;
    
    // 输入输出属性 - 使用 in-out 关键字标记可读写属性
    in-out property <CustomerData> selected-customer: {};
    in-out property <int> current-page: 0;
    in-out property <int> page-size: 20;
    
    // 回调函数 - 定义组件对外的事件接口
    callback customer-selected(CustomerData);
    callback customer-edited(CustomerData);
    callback customer-deleted(CustomerData);
    callback refresh-requested();
    callback search-changed(string);
    callback filter-changed(CustomerLevel);
    callback page-changed(int);
    
    // 私有属性 - 组件内部使用的状态
    private property <bool> show-details: false;
    private property <CustomerData> hover-customer: {};
    
    // 主布局容器
    VerticalBox {
        spacing: 16px;
        padding: 16px;
        
        // 顶部工具栏
        toolbar := HorizontalBox {
            spacing: 12px;
            alignment: space-between;
            
            // 左侧搜索和筛选区域
            search-filter-area := HorizontalBox {
                spacing: 8px;
                
                // 搜索框
                search-input := LineEdit {
                    placeholder-text: "搜索客户名称、电话...";
                    text: search-keyword;
                    width: 240px;
                    
                    // 搜索框文本变化时触发回调
                    edited(text) => {
                        search-changed(text);
                    }
                }
                
                // 等级筛选下拉框
                level-filter := ComboBox {
                    model: ["全部", "VIP客户", "重要客户", "普通客户", "潜在客户"];
                    current-index: filter-level == CustomerLevel.all ? 0 :
                                  filter-level == CustomerLevel.vip ? 1 :
                                  filter-level == CustomerLevel.important ? 2 :
                                  filter-level == CustomerLevel.normal ? 3 : 4;
                    width: 120px;
                    
                    // 筛选条件变化时触发回调
                    selected(index) => {
                        filter-changed(
                            index == 0 ? CustomerLevel.all :
                            index == 1 ? CustomerLevel.vip :
                            index == 2 ? CustomerLevel.important :
                            index == 3 ? CustomerLevel.normal :
                            CustomerLevel.potential
                        );
                    }
                }
            }
            
            // 右侧操作按钮区域
            action-buttons := HorizontalBox {
                spacing: 8px;
                
                // 刷新按钮
                refresh-button := Button {
                    text: "🔄 刷新";
                    enabled: !loading;
                    
                    clicked => {
                        refresh-requested();
                    }
                }
                
                // 新增客户按钮
                add-button := Button {
                    text: "➕ 新增客户";
                    primary: true;
                    
                    clicked => {
                        // 触发新增客户对话框
                        customer-edited({});
                    }
                }
            }
        }
        
        // 客户列表区域
        list-container := Rectangle {
            background: white;
            border-radius: 8px;
            border-width: 1px;
            border-color: #e0e0e0;
            
            VerticalBox {
                // 列表头部
                list-header := Rectangle {
                    height: 44px;
                    background: #f5f5f5;
                    border-bottom: 1px solid #e0e0e0;
                    
                    HorizontalBox {
                        padding-left: 16px;
                        padding-right: 16px;
                        alignment: start;
                        
                        // 表头列定义
                        header-name := Text {
                            text: "客户名称";
                            font-weight: 600;
                            width: 200px;
                            vertical-alignment: center;
                        }
                        
                        header-phone := Text {
                            text: "联系电话";
                            font-weight: 600;
                            width: 150px;
                            vertical-alignment: center;
                        }
                        
                        header-company := Text {
                            text: "公司名称";
                            font-weight: 600;
                            width: 200px;
                            vertical-alignment: center;
                        }
                        
                        header-level := Text {
                            text: "客户等级";
                            font-weight: 600;
                            width: 100px;
                            vertical-alignment: center;
                        }
                        
                        header-created := Text {
                            text: "创建时间";
                            font-weight: 600;
                            width: 120px;
                            vertical-alignment: center;
                        }
                        
                        header-actions := Text {
                            text: "操作";
                            font-weight: 600;
                            width: 120px;
                            vertical-alignment: center;
                        }
                    }
                }
                
                // 客户列表内容
                if loading: Rectangle {
                    height: 200px;
                    
                    // 加载指示器
                    loading-indicator := VerticalBox {
                        alignment: center;
                        
                        Text {
                            text: "正在加载客户数据...";
                            font-size: 14px;
                            color: #666;
                        }
                        
                        // 这里可以添加加载动画
                        Rectangle {
                            width: 32px;
                            height: 32px;
                            border-radius: 16px;
                            border-width: 3px;
                            border-color: #007bff;
                            
                            // 旋转动画
                            animate rotation-angle {
                                duration: 1s;
                                iteration-count: -1;
                            }
                        }
                    }
                }
                
                if !loading: ScrollView {
                    ListView {
                        for customer[index] in customers: customer-item := Rectangle {
                            height: 56px;
                            background: index % 2 == 0 ? white : #fafafa;
                            
                            // 鼠标悬停效果
                            states [
                                hover when customer-item-touch.has-hover: {
                                    background: #f0f8ff;
                                }
                                selected when selected-customer.id == customer.id: {
                                    background: #e3f2fd;
                                    border-left: 4px solid #2196f3;
                                }
                            ]
                            
                            HorizontalBox {
                                padding-left: 16px;
                                padding-right: 16px;
                                alignment: start;
                                
                                // 客户名称列
                                name-column := VerticalBox {
                                    width: 200px;
                                    alignment: center;
                                    
                                    Text {
                                        text: customer.name;
                                        font-size: 14px;
                                        font-weight: 500;
                                        color: #212529;
                                        overflow: elide;
                                    }
                                }
                                
                                // 联系电话列
                                phone-column := VerticalBox {
                                    width: 150px;
                                    alignment: center;
                                    
                                    Text {
                                        text: customer.phone;
                                        font-size: 14px;
                                        color: #495057;
                                        overflow: elide;
                                    }
                                }
                                
                                // 公司名称列
                                company-column := VerticalBox {
                                    width: 200px;
                                    alignment: center;
                                    
                                    Text {
                                        text: customer.company != "" ? customer.company : "-";
                                        font-size: 14px;
                                        color: #6c757d;
                                        overflow: elide;
                                    }
                                }
                                
                                // 客户等级列
                                level-column := VerticalBox {
                                    width: 100px;
                                    alignment: center;
                                    
                                    // 等级标签
                                    level-badge := Rectangle {
                                        width: 60px;
                                        height: 24px;
                                        border-radius: 12px;
                                        background: customer.level == CustomerLevel.vip ? #ff6b6b :
                                                   customer.level == CustomerLevel.important ? #4ecdc4 :
                                                   customer.level == CustomerLevel.normal ? #45b7d1 :
                                                   #96ceb4;
                                        
                                        Text {
                                            text: customer.level == CustomerLevel.vip ? "VIP" :
                                                 customer.level == CustomerLevel.important ? "重要" :
                                                 customer.level == CustomerLevel.normal ? "普通" :
                                                 "潜在";
                                            font-size: 12px;
                                            color: white;
                                            font-weight: 500;
                                            horizontal-alignment: center;
                                            vertical-alignment: center;
                                        }
                                    }
                                }
                                
                                // 创建时间列
                                created-column := VerticalBox {
                                    width: 120px;
                                    alignment: center;
                                    
                                    Text {
                                        text: format-date(customer.created-at);
                                        font-size: 12px;
                                        color: #6c757d;
                                    }
                                }
                                
                                // 操作按钮列
                                actions-column := HorizontalBox {
                                    width: 120px;
                                    spacing: 4px;
                                    alignment: center;
                                    
                                    // 编辑按钮
                                    edit-button := Button {
                                        text: "✏️";
                                        width: 32px;
                                        height: 32px;
                                        
                                        clicked => {
                                            customer-edited(customer);
                                        }
                                    }
                                    
                                    // 删除按钮
                                    delete-button := Button {
                                        text: "🗑️";
                                        width: 32px;
                                        height: 32px;
                                        
                                        clicked => {
                                            customer-deleted(customer);
                                        }
                                    }
                                }
                            }
                            
                            // 点击区域
                            customer-item-touch := TouchArea {
                                clicked => {
                                    selected-customer = customer;
                                    customer-selected(customer);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // 分页控件
        if !loading && customers.length > 0: pagination := HorizontalBox {
            alignment: center;
            spacing: 8px;
            
            // 上一页按钮
            prev-button := Button {
                text: "‹ 上一页";
                enabled: current-page > 0;
                
                clicked => {
                    if (current-page > 0) {
                        page-changed(current-page - 1);
                    }
                }
            }
            
            // 页码信息
            page-info := Text {
                text: "第 " + (current-page + 1) + " 页";
                font-size: 14px;
                color: #495057;
                vertical-alignment: center;
            }
            
            // 下一页按钮
            next-button := Button {
                text: "下一页 ›";
                enabled: customers.length >= page-size;
                
                clicked => {
                    page-changed(current-page + 1);
                }
            }
        }
        
        // 空状态提示
        if !loading && customers.length == 0: empty-state := VerticalBox {
            alignment: center;
            spacing: 16px;
            height: 200px;
            
            Text {
                text: "📋";
                font-size: 48px;
            }
            
            Text {
                text: search-keyword != "" ? "未找到匹配的客户" : "暂无客户数据";
                font-size: 16px;
                color: #6c757d;
            }
            
            if search-keyword == "": Button {
                text: "➕ 添加第一个客户";
                primary: true;
                
                clicked => {
                    customer-edited({});
                }
            }
        }
    }
}

// 客户数据结构定义
export struct CustomerData {
    id: int,
    name: string,
    phone: string,
    email: string,
    company: string,
    level: CustomerLevel,
    created-at: string,
    updated-at: string,
}

// 客户等级枚举
export enum CustomerLevel {
    all,        // 全部（用于筛选）
    vip,        // VIP客户
    important,  // 重要客户
    normal,     // 普通客户
    potential,  // 潜在客户
}

// 辅助函数：格式化日期
pure function format-date(date-string: string) -> string {
    // 这里需要实现日期格式化逻辑
    // 实际实现中可能需要从Rust后端获取格式化后的日期
    date-string
}
```

### 2. Slint样式和主题规范

```slint
// src/presentation/ui/themes/default_theme.slint

/// 默认主题定义
/// 
/// 定义应用程序的颜色、字体、间距等视觉样式。
/// 支持浅色和深色主题切换。
export global AppTheme {
    // 主题切换状态
    in-out property <bool> dark-mode: false;
    
    // 颜色定义
    in-out property <color> primary-color: dark-mode ? #4a9eff : #007bff;
    in-out property <color> primary-light: dark-mode ? #1e3a5f : #e3f2fd;
    in-out property <color> primary-dark: dark-mode ? #0d47a1 : #1976d2;
    
    // 背景颜色
    in-out property <color> background-color: dark-mode ? #121212 : #ffffff;
    in-out property <color> surface-color: dark-mode ? #1e1e1e : #f8f9fa;
    in-out property <color> card-color: dark-mode ? #2a2a2a : #ffffff;
    
    // 文字颜色
    in-out property <color> text-primary: dark-mode ? #ffffff : #212529;
    in-out property <color> text-secondary: dark-mode ? #b0b0b0 : #6c757d;
    in-out property <color> text-disabled: dark-mode ? #666666 : #adb5bd;
    
    // 边框和分割线
    in-out property <color> border-color: dark-mode ? #333333 : #dee2e6;
    in-out property <color> divider-color: dark-mode ? #2a2a2a : #e9ecef;
    
    // 状态颜色
    in-out property <color> success-color: dark-mode ? #4caf50 : #28a745;
    in-out property <color> warning-color: dark-mode ? #ff9800 : #ffc107;
    in-out property <color> error-color: dark-mode ? #f44336 : #dc3545;
    in-out property <color> info-color: dark-mode ? #2196f3 : #17a2b8;
    
    // 字体定义
    in-out property <string> font-family: "Microsoft YaHei UI, Arial, sans-serif";
    in-out property <length> font-size-small: 12px;
    in-out property <length> font-size-normal: 14px;
    in-out property <length> font-size-large: 16px;
    in-out property <length> font-size-heading: 18px;
    in-out property <length> font-size-title: 24px;
    
    // 间距定义
    in-out property <length> spacing-xs: 4px;
    in-out property <length> spacing-sm: 8px;
    in-out property <length> spacing-md: 16px;
    in-out property <length> spacing-lg: 24px;
    in-out property <length> spacing-xl: 32px;
    
    // 圆角定义
    in-out property <length> border-radius-sm: 4px;
    in-out property <length> border-radius-md: 8px;
    in-out property <length> border-radius-lg: 12px;
    
    // 阴影定义
    in-out property <color> shadow-color: dark-mode ? rgba(0, 0, 0, 0.5) : rgba(0, 0, 0, 0.1);
    
    // 动画时长
    in-out property <duration> animation-fast: 150ms;
    in-out property <duration> animation-normal: 250ms;
    in-out property <duration> animation-slow: 350ms;
}

/// 通用按钮样式组件
export component ThemedButton inherits Rectangle {
    in property <string> text: "";
    in property <bool> primary: false;
    in property <bool> enabled: true;
    in property <bool> loading: false;
    
    callback clicked();
    
    // 按钮样式
    background: primary ? AppTheme.primary-color : AppTheme.surface-color;
    border-radius: AppTheme.border-radius-sm;
    border-width: primary ? 0 : 1px;
    border-color: AppTheme.border-color;
    
    // 状态样式
    states [
        disabled when !enabled: {
            background: AppTheme.text-disabled;
            opacity: 0.6;
        }
        hover when touch-area.has-hover && enabled: {
            background: primary ? AppTheme.primary-dark : AppTheme.primary-light;
        }
        pressed when touch-area.pressed && enabled: {
            background: primary ? AppTheme.primary-dark : AppTheme.border-color;
        }
    ]
    
    // 动画效果
    animate background { duration: AppTheme.animation-fast; }
    animate opacity { duration: AppTheme.animation-fast; }
    
    // 按钮内容
    HorizontalBox {
        alignment: center;
        padding: 8px 16px;
        
        if loading: Rectangle {
            width: 16px;
            height: 16px;
            border-radius: 8px;
            border-width: 2px;
            border-color: primary ? white : AppTheme.primary-color;
            
            animate rotation-angle {
                duration: 1s;
                iteration-count: -1;
            }
        }
        
        if !loading: Text {
            text: text;
            color: primary ? white : AppTheme.text-primary;
            font-size: AppTheme.font-size-normal;
            font-family: AppTheme.font-family;
            vertical-alignment: center;
        }
    }
    
    // 点击区域
    touch-area := TouchArea {
        enabled: enabled && !loading;
        
        clicked => {
            if (enabled && !loading) {
                clicked();
            }
        }
    }
}

/// 通用输入框样式组件
export component ThemedLineEdit inherits Rectangle {
    in-out property <string> text: "";
    in property <string> placeholder-text: "";
    in property <bool> enabled: true;
    in property <bool> has-error: false;
    
    callback edited(string);
    callback accepted(string);
    
    // 输入框样式
    background: AppTheme.card-color;
    border-radius: AppTheme.border-radius-sm;
    border-width: 1px;
    border-color: has-error ? AppTheme.error-color : 
                  line-edit.has-focus ? AppTheme.primary-color : 
                  AppTheme.border-color;
    
    // 状态样式
    states [
        disabled when !enabled: {
            background: AppTheme.surface-color;
            opacity: 0.6;
        }
    ]
    
    // 动画效果
    animate border-color { duration: AppTheme.animation-fast; }
    
    // 输入框内容
    HorizontalBox {
        padding: 8px 12px;
        
        line-edit := LineEdit {
            text: text;
            placeholder-text: placeholder-text;
            enabled: enabled;
            color: AppTheme.text-primary;
            font-size: AppTheme.font-size-normal;
            font-family: AppTheme.font-family;
            background: transparent;
            border: none;
            
            edited(new-text) => {
                text = new-text;
                edited(new-text);
            }
            
            accepted(accepted-text) => {
                accepted(accepted-text);
            }
        }
    }
}
```