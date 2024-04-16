-- Sample data for Users table
INSERT INTO Users (email, password, verified, created_at, updated_at)
VALUES
    ('user1@example.com', 'hashed_password_1', true, '2023-01-15', '2023-01-15'),
    ('user2@example.com', 'hashed_password_2', true, '2023-01-16', '2023-01-16'),
    ('user3@example.com', 'hashed_password_3', true, '2023-01-17', '2023-01-17');

-- Sample data for Workspaces table
INSERT INTO Workspaces (user_id, name, url_slug, created_at, updated_at)
VALUES
    (1, 'Project A Workspace', 'project-a', '2023-01-15', '2023-01-15'),
    (2, 'Project B Workspace', 'project-b', '2023-01-16', '2023-01-16');

-- Sample data for Issues table
INSERT INTO Issues (workspace_id, name, description, status, priority, assignee_id, created_at, updated_at)
VALUES
    (1, 'Task 1', 'Description for Task 1', 1, 2, 1, '2023-01-15', '2023-01-15'),
    (1, 'Task 2', 'Description for Task 2', 0, 3, 2, '2023-01-16', '2023-01-16'),
    (2, 'Task 3', 'Description for Task 3', 2, 1, 3, '2023-01-17', '2023-01-17');

-- Sample data for Labels table
INSERT INTO Labels (workspace_id, name, color_code)
VALUES
    (1, 'Bug', '#FF0000'),
    (1, 'Feature', '#00FF00'),
    (2, 'Enhancement', '#0000FF');

-- Sample data for SubIssues table
INSERT INTO SubIssues (issue_id, parent_id, name, description, status, priority, assignee_id, created_at, updated_at)
VALUES
    (1, NULL, 'Sub-task 1', 'Description for Sub-task 1', 0, 2, 2, '2023-01-15', '2023-01-15'),
    (1, NULL, 'Sub-task 2', 'Description for Sub-task 2', 1, 3, 1, '2023-01-16', '2023-01-16');

-- Sample data for Teams table
INSERT INTO Teams (workspace_id, name, created_at, updated_at)
VALUES
    (1, 'Team A', '2023-01-15', '2023-01-15'),
    (2, 'Team B', '2023-01-16', '2023-01-16');

-- Sample data for Roles table
INSERT INTO Roles (name, description)
VALUES
    ('Admin', 'Admin role with full access'),
    ('Member', 'Regular member role'),
    ('Viewer', 'Role with read-only access');

-- Sample data for TeamMembers table
INSERT INTO TeamMembers (team_id, user_id, role_id, created_at, updated_at)
VALUES
    (1, 1, 1, '2023-01-15', '2023-01-15'),
    (1, 2, 2, '2023-01-16', '2023-01-16'),
    (2, 3, 3, '2023-01-17', '2023-01-17');
