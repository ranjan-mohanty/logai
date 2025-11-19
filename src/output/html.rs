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
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
            transition: background 0.3s ease;
        }}
        
        body.dark-mode {{
            background: linear-gradient(135deg, #1a202c 0%, #2d3748 100%);
        }}
        
        .container {{
            max-width: 1400px;
            margin: 0 auto;
        }}
        
        .header {{
            background: white;
            border-radius: 12px;
            padding: 30px;
            margin-bottom: 20px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
            position: relative;
            transition: background 0.3s ease, color 0.3s ease;
        }}
        
        body.dark-mode .header {{
            background: #2d3748;
        }}
        
        .header h1 {{
            color: #2d3748;
            font-size: 32px;
            margin-bottom: 10px;
            transition: color 0.3s ease;
        }}
        
        body.dark-mode .header h1 {{
            color: #e2e8f0;
        }}
        
        .header .subtitle {{
            color: #718096;
            font-size: 14px;
            transition: color 0.3s ease;
        }}
        
        body.dark-mode .header .subtitle {{
            color: #a0aec0;
        }}
        
        .theme-toggle {{
            position: absolute;
            top: 30px;
            right: 30px;
            background: #667eea;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 8px;
            cursor: pointer;
            font-size: 14px;
            font-weight: 500;
            transition: background 0.2s;
            display: flex;
            align-items: center;
            gap: 8px;
        }}
        
        .theme-toggle:hover {{
            background: #5a67d8;
        }}
        
        body.dark-mode .theme-toggle {{
            background: #4a5568;
        }}
        
        body.dark-mode .theme-toggle:hover {{
            background: #2d3748;
        }}
        
        .stats {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
            margin-bottom: 20px;
        }}
        
        .stat-card {{
            background: white;
            border-radius: 12px;
            padding: 20px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
            transition: background 0.3s ease;
        }}
        
        body.dark-mode .stat-card {{
            background: #2d3748;
        }}
        
        .stat-card .label {{
            color: #718096;
            transition: color 0.3s ease;
        }}
        
        body.dark-mode .stat-card .label {{
            color: #a0aec0;
            font-size: 14px;
            margin-bottom: 5px;
        }}
        
        .stat-card .value {{
            color: #2d3748;
            font-size: 28px;
            font-weight: bold;
            transition: color 0.3s ease;
        }}
        
        body.dark-mode .stat-card .value {{
            color: #e2e8f0;
        }}
        
        .error-group {{
            background: white;
            border-radius: 12px;
            padding: 25px;
            margin-bottom: 15px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
            transition: transform 0.2s, box-shadow 0.2s, background 0.3s ease;
            cursor: pointer;
        }}
        
        body.dark-mode .error-group {{
            background: #2d3748;
        }}
        
        .error-group:hover {{
            transform: translateY(-2px);
            box-shadow: 0 6px 12px rgba(0, 0, 0, 0.15);
        }}
        
        .error-header {{
            display: flex;
            justify-content: space-between;
            align-items: start;
            margin-bottom: 15px;
        }}
        
        .error-title {{
            flex: 1;
            color: #2d3748;
            font-size: 18px;
            font-weight: 600;
            line-height: 1.4;
            transition: color 0.3s ease;
        }}
        
        body.dark-mode .error-title {{
            color: #e2e8f0;
        }}
        
        .error-badge {{
            background: #fc8181;
            color: white;
            padding: 6px 12px;
            border-radius: 20px;
            font-size: 14px;
            font-weight: 600;
            margin-left: 15px;
            white-space: nowrap;
        }}
        
        .error-meta {{
            display: flex;
            gap: 20px;
            margin-bottom: 15px;
            flex-wrap: wrap;
        }}
        
        .meta-item {{
            color: #718096;
            font-size: 14px;
            transition: color 0.3s ease;
        }}
        
        body.dark-mode .meta-item {{
            color: #a0aec0;
        }}
        
        .meta-item strong {{
            color: #4a5568;
            transition: color 0.3s ease;
        }}
        
        body.dark-mode .meta-item strong {{
            color: #cbd5e0;
        }}
        
        .error-example {{
            background: #f7fafc;
            border-left: 4px solid #667eea;
            padding: 15px;
            border-radius: 6px;
            margin-top: 15px;
            margin-bottom: 15px;
            display: none;
            transition: background 0.3s ease;
        }}
        
        body.dark-mode .error-example {{
            background: #1a202c;
        }}
        
        .error-example.expanded {{
            display: block;
        }}
        
        .error-example pre {{
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
            font-size: 13px;
            color: #2d3748;
            white-space: pre-wrap;
            word-wrap: break-word;
            transition: color 0.3s ease;
        }}
        
        body.dark-mode .error-example pre {{
            color: #e2e8f0;
        }}
        
        .analysis-section {{
            display: none;
            margin-top: 15px;
            padding-top: 15px;
            border-top: 1px solid #e2e8f0;
            transition: border-color 0.3s ease;
        }}
        
        body.dark-mode .analysis-section {{
            border-top-color: #4a5568;
        }}
        
        .analysis-section.expanded {{
            display: block;
        }}
        
        .occurrences-section {{
            display: none;
            margin-top: 15px;
            padding-top: 15px;
            border-top: 1px solid #e2e8f0;
            max-height: 400px;
            overflow-y: auto;
            transition: border-color 0.3s ease;
        }}
        
        body.dark-mode .occurrences-section {{
            border-top-color: #4a5568;
        }}
        
        .occurrences-section.expanded {{
            display: block;
        }}
        
        .occurrence-item {{
            background: #f7fafc;
            border-left: 3px solid #667eea;
            padding: 12px;
            margin-bottom: 10px;
            border-radius: 6px;
            transition: background 0.3s ease;
        }}
        
        body.dark-mode .occurrence-item {{
            background: #1a202c;
        }}
        
        .occurrence-header {{
            display: flex;
            justify-content: space-between;
            margin-bottom: 8px;
            font-size: 13px;
        }}
        
        .occurrence-number {{
            color: #667eea;
            font-weight: 600;
        }}
        
        .occurrence-time {{
            color: #718096;
            transition: color 0.3s ease;
        }}
        
        body.dark-mode .occurrence-time {{
            color: #a0aec0;
        }}
        
        .occurrence-message {{
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
            font-size: 12px;
            color: #2d3748;
            white-space: pre-wrap;
            word-wrap: break-word;
            margin: 0;
            transition: color 0.3s ease;
        }}
        
        body.dark-mode .occurrence-message {{
            color: #e2e8f0;
        }}
        
        .analysis-title {{
            color: #4a5568;
            font-size: 16px;
            font-weight: 600;
            margin-bottom: 10px;
            transition: color 0.3s ease;
        }}
        
        body.dark-mode .analysis-title {{
            color: #cbd5e0;
        }}
        
        .analysis-content {{
            color: #2d3748;
            line-height: 1.6;
            margin-bottom: 15px;
            transition: color 0.3s ease;
        }}
        
        body.dark-mode .analysis-content {{
            color: #e2e8f0;
        }}
        
        .suggestions {{
            list-style: none;
        }}
        
        .suggestion {{
            background: #f0fff4;
            border-left: 4px solid #48bb78;
            padding: 12px;
            margin-bottom: 10px;
            border-radius: 6px;
        }}
        
        .suggestion-priority {{
            display: inline-block;
            background: #48bb78;
            color: white;
            padding: 2px 8px;
            border-radius: 12px;
            font-size: 12px;
            font-weight: 600;
            margin-right: 8px;
        }}
        
        .suggestion-priority.high {{
            background: #f56565;
        }}
        
        .suggestion-priority.medium {{
            background: #ed8936;
        }}
        
        .code-example {{
            background: #2d3748;
            color: #e2e8f0;
            padding: 12px;
            border-radius: 6px;
            margin-top: 8px;
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
            font-size: 13px;
            overflow-x: auto;
        }}
        
        .expand-btn {{
            background: #667eea;
            color: white;
            border: none;
            padding: 8px 16px;
            border-radius: 6px;
            cursor: pointer;
            font-size: 14px;
            font-weight: 500;
            transition: background 0.2s;
        }}
        
        .expand-btn:hover {{
            background: #5a67d8;
        }}
        
        .search-box {{
            background: white;
            border-radius: 12px;
            padding: 20px;
            margin-bottom: 20px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
            transition: background 0.3s ease;
        }}
        
        body.dark-mode .search-box {{
            background: #2d3748;
        }}
        
        .search-input {{
            width: 100%;
            padding: 12px;
            border: 2px solid #e2e8f0;
            border-radius: 8px;
            font-size: 16px;
            transition: border-color 0.2s, background 0.3s ease, color 0.3s ease;
            background: white;
            color: #2d3748;
        }}
        
        body.dark-mode .search-input {{
            background: #1a202c;
            border-color: #4a5568;
            color: #e2e8f0;
        }}
        
        .search-input:focus {{
            outline: none;
            border-color: #667eea;
        }}
        
        .no-results {{
            text-align: center;
            padding: 40px;
            color: #718096;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <button class="theme-toggle" onclick="toggleTheme()">
                <span id="themeIcon">🌙</span>
                <span id="themeText">Dark Mode</span>
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
                themeIcon.textContent = '☀️';
                themeText.textContent = 'Light Mode';
                localStorage.setItem('theme', 'dark');
            }} else {{
                themeIcon.textContent = '🌙';
                themeText.textContent = 'Dark Mode';
                localStorage.setItem('theme', 'light');
            }}
        }}
        
        // Load saved theme preference
        document.addEventListener('DOMContentLoaded', () => {{
            const savedTheme = localStorage.getItem('theme');
            if (savedTheme === 'dark') {{
                document.body.classList.add('dark-mode');
                document.getElementById('themeIcon').textContent = '☀️';
                document.getElementById('themeText').textContent = 'Light Mode';
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
