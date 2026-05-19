CREATE TABLE IF NOT EXISTS rate_plan_packages (
    plan_code TEXT NOT NULL,
    package_code TEXT NOT NULL,
    sort_order INTEGER NOT NULL,
    PRIMARY KEY (plan_code, package_code),
    FOREIGN KEY(package_code)
        REFERENCES package_definitions(package_code)
);
