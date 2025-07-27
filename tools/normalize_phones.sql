-- Script to normalize existing phone numbers in the database
-- This will convert various formats to the standard 11-digit format:
-- Mobile: 09XXXXXXXXX (11 digits total)
-- Landline: 0XXXXXXXXXX (11 digits total with area code)

-- First, let's see what formats we have
SELECT DISTINCT
    LENGTH(phone_number) as len,
    SUBSTR(phone_number, 1, 4) as prefix,
    COUNT(*) as count
FROM customers
GROUP BY LENGTH(phone_number), SUBSTR(phone_number, 1, 4)
ORDER BY count DESC;

-- Normalize phone numbers
UPDATE customers
SET phone_number =
    CASE
        -- Already in correct format (11 digits starting with 0)
        WHEN LENGTH(REPLACE(REPLACE(REPLACE(phone_number, ' ', ''), '-', ''), '()', '')) = 11
             AND SUBSTR(REPLACE(REPLACE(REPLACE(phone_number, ' ', ''), '-', ''), '()', ''), 1, 1) = '0'
        THEN REPLACE(REPLACE(REPLACE(phone_number, ' ', ''), '-', ''), '()', '')

        -- Missing 0 prefix for mobile (10 digits starting with 9)
        WHEN LENGTH(REPLACE(REPLACE(REPLACE(phone_number, ' ', ''), '-', ''), '()', '')) = 10
             AND SUBSTR(REPLACE(REPLACE(REPLACE(phone_number, ' ', ''), '-', ''), '()', ''), 1, 1) = '9'
        THEN '0' || REPLACE(REPLACE(REPLACE(phone_number, ' ', ''), '-', ''), '()', '')

        -- International format with +98
        WHEN SUBSTR(phone_number, 1, 3) = '+98'
        THEN '0' || SUBSTR(REPLACE(REPLACE(REPLACE(phone_number, ' ', ''), '-', ''), '()', ''), 4)

        -- International format with 98
        WHEN LENGTH(REPLACE(REPLACE(REPLACE(phone_number, ' ', ''), '-', ''), '()', '')) = 12
             AND SUBSTR(REPLACE(REPLACE(REPLACE(phone_number, ' ', ''), '-', ''), '()', ''), 1, 2) = '98'
        THEN '0' || SUBSTR(REPLACE(REPLACE(REPLACE(phone_number, ' ', ''), '-', ''), '()', ''), 3)

        -- Keep as is if none of the above patterns match
        ELSE phone_number
    END
WHERE phone_number IS NOT NULL;

-- Verify the results
SELECT
    phone_number,
    LENGTH(phone_number) as len,
    CASE
        WHEN LENGTH(phone_number) = 11 AND SUBSTR(phone_number, 1, 2) = '09' THEN '✓ Valid Mobile'
        WHEN LENGTH(phone_number) = 11 AND SUBSTR(phone_number, 1, 1) = '0' THEN '✓ Valid Landline'
        ELSE '✗ Invalid'
    END as status
FROM customers
ORDER BY status DESC, phone_number;
