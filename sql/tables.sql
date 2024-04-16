-- Create Users table
CREATE TABLE Users (
    id SERIAL PRIMARY KEY,
	full_name VARCHAR(100),
	username VARCHAR(50) UNIQUE,
	profile_picture VARCHAR(255),
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL, -- Hashed password
    verification_code VARCHAR(50),
    verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create Workspaces table
CREATE TABLE Workspaces (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES Users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    url_slug VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create Issues table
CREATE TABLE Issues (
    id SERIAL PRIMARY KEY,
    workspace_id INTEGER REFERENCES Workspaces(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    status INTEGER DEFAULT 0, -- Use ENUM or integer values for status
    priority INTEGER DEFAULT 0, -- Use ENUM or integer values for priority
	assignee_id INTEGER REFERENCES Users(id),
	team_id INTEGER REFERENCES Teams(id) ON DELETE CASCADE,
    created_by INTEGER REFERENCES Users(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create Labels table
CREATE TABLE Labels (
    id SERIAL PRIMARY KEY,
    workspace_id INTEGER REFERENCES Workspaces(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    color_code VARCHAR(20) NOT NULL
);

-- Create SubIssues table
CREATE TABLE SubIssues (
    id SERIAL PRIMARY KEY,
    issue_id INTEGER REFERENCES Issues(id) ON DELETE CASCADE,
    parent_id INTEGER REFERENCES Issues(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    status INTEGER DEFAULT 0, -- Use ENUM or integer values for status
    priority INTEGER DEFAULT 0, -- Use ENUM or integer values for priority
    assignee_id INTEGER REFERENCES Users(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create Teams table
CREATE TABLE Teams (
    id SERIAL PRIMARY KEY,
    workspace_id INTEGER REFERENCES Workspaces(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create Roles table
CREATE TABLE Roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    description TEXT
);

-- Create Team Members table
CREATE TABLE TeamMembers (
    id SERIAL PRIMARY KEY,
    team_id INTEGER REFERENCES Teams(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES Users(id) ON DELETE CASCADE,
    role_id INTEGER REFERENCES Roles(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_preference (
    user_id SERIAL PRIMARY KEY,
    display_full_name BOOLEAN,
    font_size VARCHAR(10), -- Assuming limited options for font size
    theme VARCHAR(10) -- Assuming limited options for theme (dark, light, pale, etc.)
);


