use crate::output::OutputFormatter;
use crate::types::ErrorGroup;
use crate::Result;
use chrono::{DateTime, Utc};

pub struct HtmlFormatter {
    limit: usize,
}

impl HtmlFormatter {
    pub fn new(limit: usize) -> Self {
        Self { limit }
    }

    fn generate_html(&self, groups: &[ErrorGroup]) -> String {
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        let total_errors: usize = groups.iter().map(|g| g.count).sum();
        let groups_to_show = groups.iter().take(self.limit).collect::<Vec<_>>();

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>LogAI Analysis Report</title>
    <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&family=Roboto+Mono:wght@400;500&display=swap" rel="stylesheet">
    <link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet">
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: 'Roboto', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: #f5f5f5;
            min-height: 100vh;
            padding: 24px;
            transition: background 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode {{
            background: #121212;
        }}
        
        .container {{
            max-width: 1400px;
            margin: 0 auto;
        }}
        
        .header {{
            background: white;
            border-radius: 8px;
            padding: 32px;
            margin-bottom: 24px;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06);
            position: relative;
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .header {{
            background: #1e1e1e;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3), 0 1px 2px rgba(0, 0, 0, 0.2);
        }}
        
        .header h1 {{
            color: #1a1a1a;
            font-size: 34px;
            font-weight: 500;
            margin-bottom: 8px;
            letter-spacing: -0.5px;
            transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .header h1 {{
            color: #ffffff;
        }}
        
        .header .subtitle {{
            color: #757575;
            font-size: 14px;
            font-weight: 400;
            transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .header .subtitle {{
            color: #b0b0b0;
        }}
        
        .theme-toggle {{
            position: absolute;
            top: 32px;
            right: 32px;
            background: transparent;
            color: #1a1a1a;
            border: none;
            padding: 8px;
            border-radius: 50%;
            cursor: pointer;
            font-size: 24px;
            transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
            display: flex;
            align-items: center;
            justify-content: center;
            width: 40px;
            height: 40px;
        }}
        
        .theme-toggle:hover {{
            background: rgba(0, 0, 0, 0.04);
        }}
        
        body.dark-mode .theme-toggle {{
            color: #ffffff;
        }}
        
        body.dark-mode .theme-toggle:hover {{
            background: rgba(255, 255, 255, 0.08);
        }}
        
        .material-icons {{
            font-size: 24px;
        }}
        
        .stats {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
            margin-bottom: 20px;
        }}
        
        .stat-card {{
            background: white;
            border-radius: 8px;
            padding: 24px;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06);
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .stat-card {{
            background: #1e1e1e;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3), 0 1px 2px rgba(0, 0, 0, 0.2);
        }}
        
        .stat-card .label {{
            color: #757575;
            font-size: 14px;
            font-weight: 500;
            margin-bottom: 8px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .stat-card .label {{
            color: #b0b0b0;
        }}
        
        .stat-card .value {{
            color: #1a1a1a;
            font-size: 32px;
            font-weight: 500;
            transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .stat-card .value {{
            color: #ffffff;
        }}
        
        .error-group {{
            background: white;
            border-radius: 8px;
            padding: 24px;
            margin-bottom: 16px;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06);
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            cursor: pointer;
            border-left: 4px solid #f44336;
        }}
        
        body.dark-mode .error-group {{
            background: #1e1e1e;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3), 0 1px 2px rgba(0, 0, 0, 0.2);
        }}
        
        .error-group:hover {{
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15), 0 2px 4px rgba(0, 0, 0, 0.1);
            transform: translateY(-1px);
        }}
        
        .error-header {{
            display: flex;
            justify-content: space-between;
            align-items: start;
            margin-bottom: 15px;
        }}
        
        .error-title {{
            flex: 1;
            color: #1a1a1a;
            font-size: 16px;
            font-weight: 500;
            line-height: 1.5;
            transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .error-title {{
            color: #ffffff;
        }}
        
        .error-badge {{
            background: #f44336;
            color: white;
            padding: 4px 12px;
            border-radius: 16px;
            font-size: 12px;
            font-weight: 500;
            margin-left: 16px;
            white-space: nowrap;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }}
        
        .error-meta {{
            display: flex;
            gap: 20px;
            margin-bottom: 15px;
            flex-wrap: wrap;
        }}
        
        .meta-item {{
            color: #757575;
            font-size: 14px;
            transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .meta-item {{
            color: #b0b0b0;
        }}
        
        .meta-item strong {{
            color: #424242;
            font-weight: 500;
            transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .meta-item strong {{
            color: #e0e0e0;
        }}
        
        .error-example {{
            background: #fafafa;
            border-left: 4px solid #2196f3;
            padding: 16px;
            border-radius: 4px;
            margin-top: 16px;
            margin-bottom: 16px;
            display: none;
            transition: background 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .error-example {{
            background: #2a2a2a;
        }}
        
        .error-example.expanded {{
            display: block;
        }}
        
        .error-example pre {{
            font-family: 'Roboto Mono', 'Monaco', 'Menlo', monospace;
            font-size: 13px;
            color: #424242;
            white-space: pre-wrap;
            word-wrap: break-word;
            line-height: 1.6;
            transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .error-example pre {{
            color: #e0e0e0;
        }}
        
        .analysis-section {{
            display: none;
            margin-top: 16px;
            padding-top: 16px;
            border-top: 1px solid #e0e0e0;
            transition: border-color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .analysis-section {{
            border-top-color: #424242;
        }}
        
        .analysis-section.expanded {{
            display: block;
        }}
        
        .occurrences-section {{
            display: none;
            margin-top: 16px;
            padding-top: 16px;
            border-top: 1px solid #e0e0e0;
            max-height: 400px;
            overflow-y: auto;
            transition: border-color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .occurrences-section {{
            border-top-color: #424242;
        }}
        
        .occurrences-section.expanded {{
            display: block;
        }}
        
        .occurrence-item {{
            background: #fafafa;
            border-left: 3px solid #2196f3;
            padding: 16px;
            margin-bottom: 12px;
            border-radius: 4px;
            transition: background 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .occurrence-item {{
            background: #2a2a2a;
        }}
        
        .occurrence-header {{
            display: flex;
            justify-content: space-between;
            margin-bottom: 8px;
            font-size: 13px;
        }}
        
        .occurrence-number {{
            color: #2196f3;
            font-weight: 500;
        }}
        
        .occurrence-time {{
            color: #757575;
            transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .occurrence-time {{
            color: #b0b0b0;
        }}
        
        .occurrence-message {{
            font-family: 'Roboto Mono', 'Monaco', 'Menlo', monospace;
            font-size: 13px;
            color: #424242;
            white-space: pre-wrap;
            word-wrap: break-word;
            margin: 0;
            line-height: 1.6;
            transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .occurrence-message {{
            color: #e0e0e0;
        }}
        
        .analysis-title {{
            color: #424242;
            font-size: 16px;
            font-weight: 500;
            margin-bottom: 12px;
            transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .analysis-title {{
            color: #e0e0e0;
        }}
        
        .analysis-content {{
            color: #424242;
            line-height: 1.6;
            margin-bottom: 16px;
            transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .analysis-content {{
            color: #e0e0e0;
        }}
        
        .suggestions {{
            list-style: none;
        }}
        
        .suggestion {{
            background: #e8f5e9;
            border-left: 4px solid #4caf50;
            padding: 16px;
            margin-bottom: 12px;
            border-radius: 4px;
        }}
        
        .suggestion-priority {{
            display: inline-block;
            background: #4caf50;
            color: white;
            padding: 4px 12px;
            border-radius: 16px;
            font-size: 11px;
            font-weight: 500;
            margin-right: 8px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }}
        
        .suggestion-priority.high {{
            background: #f44336;
        }}
        
        .suggestion-priority.medium {{
            background: #ff9800;
        }}
        
        .code-example {{
            background: #263238;
            color: #aed581;
            padding: 16px;
            border-radius: 4px;
            margin-top: 12px;
            font-family: 'Roboto Mono', 'Monaco', 'Menlo', monospace;
            font-size: 13px;
            overflow-x: auto;
            line-height: 1.6;
        }}
        
        .expand-btn {{
            background: #2196f3;
            color: white;
            border: none;
            padding: 8px 16px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
            font-weight: 500;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
        }}
        
        .expand-btn:hover {{
            background: #1976d2;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
        }}
        
        .expand-btn:active {{
            box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
        }}
        
        .search-box {{
            background: white;
            border-radius: 8px;
            padding: 24px;
            margin-bottom: 24px;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06);
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        
        body.dark-mode .search-box {{
            background: #1e1e1e;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3), 0 1px 2px rgba(0, 0, 0, 0.2);
        }}
        
        .search-input {{
            width: 100%;
            padding: 16px;
            border: none;
            border-bottom: 2px solid #e0e0e0;
            border-radius: 4px 4px 0 0;
            font-size: 16px;
            font-family: 'Roboto', sans-serif;
            transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
            background: #fafafa;
            color: #1a1a1a;
        }}
        
        body.dark-mode .search-input {{
            background: #2a2a2a;
            border-bottom-color: #424242;
            color: #ffffff;
        }}
        
        .search-input:focus {{
            outline: none;
            border-bottom-color: #2196f3;
            background: white;
        }}
        
        body.dark-mode .search-input:focus {{
            background: #1e1e1e;
        }}
        
        .no-results {{
            text-align: center;
            padding: 48px;
            color: #757575;
        }}
        
        body.dark-mode .no-results {{
            color: #b0b0b0;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <button class="theme-toggle" onclick="toggleTheme()" title="Toggle theme">
                <span id="themeIcon" class="material-icons">dark_mode</span>
            </button>
            <h1>🤖 LogAI Analysis Report</h1>
            <div class="subtitle">Generated on {timestamp}</div>
        </div>
        
        <div class="stats">
            <div class="stat-card">
                <div class="label">Unique Error Patterns</div>
                <div class="value">{}</div>
            </div>
            <div class="stat-card">
                <div class="label">Total Occurrences</div>
                <div class="value">{}</div>
            </div>
            <div class="stat-card">
                <div class="label">Showing</div>
                <div class="value">{} / {}</div>
            </div>
        </div>
        
        <div class="search-box">
            <input type="text" class="search-input" id="searchInput" placeholder="🔍 Search errors by pattern, message, or file...">
        </div>
        
        <div id="errorGroups">
            {}
        </div>
    </div>
    
    <script>
        // Theme toggle
        function toggleTheme() {{
            const body = document.body;
            const themeIcon = document.getElementById('themeIcon');
            const themeText = document.getElementById('themeText');
            
            body.classList.toggle('dark-mode');
            
            if (body.classList.contains('dark-mode')) {{
                themeIcon.textContent = 'light_mode';
                localStorage.setItem('theme', 'dark');
            }} else {{
                themeIcon.textContent = 'dark_mode';
                localStorage.setItem('theme', 'light');
            }}
        }}
        
        // Load saved theme preference
        document.addEventListener('DOMContentLoaded', () => {{
            const savedTheme = localStorage.getItem('theme');
            if (savedTheme === 'dark') {{
                document.body.classList.add('dark-mode');
                document.getElementById('themeIcon').textContent = 'light_mode';
            }}
        }});
        
        // Toggle error details
        function toggleError(id) {{
            const example = document.getElementById('example-' + id);
            const analysis = document.getElementById('analysis-' + id);
            example.classList.toggle('expanded');
            if (analysis) {{
                analysis.classList.toggle('expanded');
            }}
        }}
        
        // Toggle occurrences
        function toggleOccurrences(id) {{
            const occurrences = document.getElementById('occurrences-' + id);
            if (occurrences) {{
                occurrences.classList.toggle('expanded');
            }}
        }}
        
        // Search functionality
        const searchInput = document.getElementById('searchInput');
        const errorGroups = document.querySelectorAll('.error-group');
        
        searchInput.addEventListener('input', (e) => {{
            const searchTerm = e.target.value.toLowerCase();
            let visibleCount = 0;
            
            errorGroups.forEach(group => {{
                const text = group.textContent.toLowerCase();
                if (text.includes(searchTerm)) {{
                    group.style.display = 'block';
                    visibleCount++;
                }} else {{
                    group.style.display = 'none';
                }}
            }});
            
            // Show no results message
            const container = document.getElementById('errorGroups');
            let noResults = document.getElementById('noResults');
            
            if (visibleCount === 0 && searchTerm) {{
                if (!noResults) {{
                    noResults = document.createElement('div');
                    noResults.id = 'noResults';
                    noResults.className = 'no-results';
                    noResults.innerHTML = '<h3>No results found</h3><p>Try a different search term</p>';
                    container.appendChild(noResults);
                }}
            }} else if (noResults) {{
                noResults.remove();
            }}
        }});
        
        // Keyboard shortcuts
        document.addEventListener('keydown', (e) => {{
            if (e.key === '/' && e.target.tagName !== 'INPUT') {{
                e.preventDefault();
                searchInput.focus();
            }}
        }});
    </script>
</body>
</html>"#,
            groups.len(),
            total_errors,
            groups_to_show.len(),
            groups.len(),
            self.generate_error_groups_html(&groups_to_show)
        )
    }

    fn generate_error_groups_html(&self, groups: &[&ErrorGroup]) -> String {
        groups
            .iter()
            .enumerate()
            .map(|(idx, group)| self.generate_error_group_html(idx, group))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn generate_error_group_html(&self, idx: usize, group: &ErrorGroup) -> String {
        let first_seen = self.format_time_ago(&group.first_seen);
        let last_seen = self.format_time_ago(&group.last_seen);

        let analysis_html = if let Some(analysis) = &group.analysis {
            format!(
                r#"
            <div class="analysis-section" id="analysis-{}">
                <div class="analysis-title">🎯 Explanation</div>
                <div class="analysis-content">{}</div>
                
                {}
                
                {}
            </div>"#,
                idx,
                self.escape_html(&analysis.explanation),
                if let Some(root_cause) = &analysis.root_cause {
                    format!(
                        r#"<div class="analysis-title">🔍 Root Cause</div>
                <div class="analysis-content">{}</div>"#,
                        self.escape_html(root_cause)
                    )
                } else {
                    String::new()
                },
                if !analysis.suggestions.is_empty() {
                    format!(
                        r#"<div class="analysis-title">💡 Suggestions</div>
                <ul class="suggestions">
                    {}
                </ul>"#,
                        analysis
                            .suggestions
                            .iter()
                            .map(|s| {
                                let priority_class = match s.priority {
                                    1..=3 => "high",
                                    4..=6 => "medium",
                                    _ => "",
                                };
                                format!(
                                    r#"<li class="suggestion">
                        <span class="suggestion-priority {}">{}</span>
                        {}
                        {}
                    </li>"#,
                                    priority_class,
                                    s.priority,
                                    self.escape_html(&s.description),
                                    if let Some(code) = &s.code_example {
                                        format!(
                                            r#"<pre class="code-example">{}</pre>"#,
                                            self.escape_html(code)
                                        )
                                    } else {
                                        String::new()
                                    }
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                } else {
                    String::new()
                }
            )
        } else {
            String::new()
        };

        format!(
            r#"<div class="error-group" onclick="toggleError({})">
            <div class="error-header">
                <div class="error-title">{}</div>
                <div class="error-badge">{} occurrence{}</div>
            </div>
            <div class="error-meta">
                <div class="meta-item"><strong>First seen:</strong> {}</div>
                <div class="meta-item"><strong>Last seen:</strong> {}</div>
                {}
            </div>
            <div style="display: flex; gap: 10px; margin-top: 15px;">
                <button class="expand-btn" onclick="event.stopPropagation(); toggleError({})">
                    View Details
                </button>
                <button class="expand-btn" onclick="event.stopPropagation(); toggleOccurrences({})">
                    Show All Occurrences
                </button>
            </div>
            <div class="error-example" id="example-{}">
                <pre>{}</pre>
            </div>
            {}
            <div class="occurrences-section" id="occurrences-{}">
                <div class="analysis-title">📋 All Occurrences</div>
                {}
            </div>
        </div>"#,
            idx,
            self.escape_html(&group.pattern),
            group.count,
            if group.count == 1 { "" } else { "s" },
            first_seen,
            last_seen,
            group
                .entries
                .first()
                .and_then(|e| e.metadata.file.as_ref())
                .map(|f| format!(
                    r#"<div class="meta-item"><strong>File:</strong> {}</div>"#,
                    self.escape_html(f)
                ))
                .unwrap_or_default(),
            idx,
            idx,
            idx,
            self.escape_html(
                group
                    .entries
                    .first()
                    .map(|e| e.message.as_str())
                    .unwrap_or("No example available")
            ),
            analysis_html,
            idx,
            self.generate_occurrences_html(group)
        )
    }

    fn generate_occurrences_html(&self, group: &ErrorGroup) -> String {
        group
            .entries
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let timestamp_str = if let Some(ts) = entry.timestamp {
                    ts.format("%Y-%m-%d %H:%M:%S UTC").to_string()
                } else {
                    // Try to extract timestamp from raw log line
                    let raw_parts: Vec<&str> = entry.raw.split_whitespace().collect();
                    if raw_parts.len() >= 2 {
                        format!("{} {}", raw_parts[0], raw_parts[1])
                    } else {
                        "No timestamp".to_string()
                    }
                };

                format!(
                    r#"<div class="occurrence-item">
                    <div class="occurrence-header">
                        <span class="occurrence-number">#{}</span>
                        <span class="occurrence-time">{}</span>
                    </div>
                    <pre class="occurrence-message">{}</pre>
                </div>"#,
                    i + 1,
                    timestamp_str,
                    self.escape_html(&entry.raw)
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_time_ago(&self, time: &DateTime<Utc>) -> String {
        time.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }

    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }
}

impl OutputFormatter for HtmlFormatter {
    fn format(&self, groups: &[ErrorGroup]) -> Result<String> {
        Ok(self.generate_html(groups))
    }
}
