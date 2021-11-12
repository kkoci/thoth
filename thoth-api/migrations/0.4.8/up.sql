-- Order is alphabetical by name of country (see string equivalents in API enum)
CREATE TYPE country_code AS ENUM (
    'afg',
    'ala',
    'alb',
    'dza',
    'asm',
    'and',
    'ago',
    'aia',
    'ata',
    'atg',
    'arg',
    'arm',
    'abw',
    'aus',
    'aut',
    'aze',
    'bhs',
    'bhr',
    'bgd',
    'brb',
    'blr',
    'bel',
    'blz',
    'ben',
    'bmu',
    'btn',
    'bol',
    'bes',
    'bih',
    'bwa',
    'bvt',
    'bra',
    'iot',
    'brn',
    'bgr',
    'bfa',
    'bdi',
    'cpv',
    'khm',
    'cmr',
    'can',
    'cym',
    'caf',
    'tcd',
    'chl',
    'chn',
    'cxr',
    'cck',
    'col',
    'com',
    'cok',
    'cri',
    'civ',
    'hrv',
    'cub',
    'cuw',
    'cyp',
    'cze',
    'cod',
    'dnk',
    'dji',
    'dma',
    'dom',
    'ecu',
    'egy',
    'slv',
    'gnq',
    'eri',
    'est',
    'swz',
    'eth',
    'flk',
    'fro',
    'fji',
    'fin',
    'fra',
    'guf',
    'pyf',
    'atf',
    'gab',
    'gmb',
    'geo',
    'deu',
    'gha',
    'gib',
    'grc',
    'grl',
    'grd',
    'glp',
    'gum',
    'gtm',
    'ggy',
    'gin',
    'gnb',
    'guy',
    'hti',
    'hmd',
    'hnd',
    'hkg',
    'hun',
    'isl',
    'ind',
    'idn',
    'irn',
    'irq',
    'irl',
    'imn',
    'isr',
    'ita',
    'jam',
    'jpn',
    'jey',
    'jor',
    'kaz',
    'ken',
    'kir',
    'kwt',
    'kgz',
    'lao',
    'lva',
    'lbn',
    'lso',
    'lbr',
    'lby',
    'lie',
    'ltu',
    'lux',
    'mac',
    'mdg',
    'mwi',
    'mys',
    'mdv',
    'mli',
    'mlt',
    'mhl',
    'mtq',
    'mrt',
    'mus',
    'myt',
    'mex',
    'fsm',
    'mda',
    'mco',
    'mng',
    'mne',
    'msr',
    'mar',
    'moz',
    'mmr',
    'nam',
    'nru',
    'npl',
    'nld',
    'ncl',
    'nzl',
    'nic',
    'ner',
    'nga',
    'niu',
    'nfk',
    'prk',
    'mkd',
    'mnp',
    'nor',
    'omn',
    'pak',
    'plw',
    'pse',
    'pan',
    'png',
    'pry',
    'per',
    'phl',
    'pcn',
    'pol',
    'prt',
    'pri',
    'qat',
    'cog',
    'reu',
    'rou',
    'rus',
    'rwa',
    'blm',
    'shn',
    'kna',
    'lca',
    'maf',
    'spm',
    'vct',
    'wsm',
    'smr',
    'stp',
    'sau',
    'sen',
    'srb',
    'syc',
    'sle',
    'sgp',
    'sxm',
    'svk',
    'svn',
    'slb',
    'som',
    'zaf',
    'sgs',
    'kor',
    'ssd',
    'esp',
    'lka',
    'sdn',
    'sur',
    'sjm',
    'swe',
    'che',
    'syr',
    'twn',
    'tjk',
    'tza',
    'tha',
    'tls',
    'tgo',
    'tkl',
    'ton',
    'tto',
    'tun',
    'tur',
    'tkm',
    'tca',
    'tuv',
    'uga',
    'ukr',
    'are',
    'gbr',
    'umi',
    'usa',
    'ury',
    'uzb',
    'vut',
    'vat',
    'ven',
    'vnm',
    'vgb',
    'vir',
    'wlf',
    'esh',
    'yem',
    'zmb',
    'zwe'
);

ALTER TABLE funder RENAME TO institution;

ALTER TABLE institution RENAME COLUMN funder_id TO institution_id;
ALTER TABLE institution RENAME COLUMN funder_name TO institution_name;
ALTER TABLE institution RENAME COLUMN funder_doi TO institution_doi;

ALTER TABLE institution
    ADD COLUMN ror TEXT CHECK (ror ~ '^https:\/\/ror\.org\/0[a-hjkmnp-z0-9]{6}\d{2}$'),
    ADD COLUMN country_code country_code;

ALTER TABLE funder_history RENAME TO institution_history;

ALTER TABLE institution_history RENAME COLUMN funder_history_id TO institution_history_id;
ALTER TABLE institution_history RENAME COLUMN funder_id TO institution_id;

ALTER TABLE funding RENAME COLUMN funder_id TO institution_id;

CREATE TABLE affiliation (
    affiliation_id      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    contribution_id     UUID NOT NULL REFERENCES contribution(contribution_id) ON DELETE CASCADE,
    institution_id      UUID NOT NULL REFERENCES institution(institution_id) ON DELETE CASCADE,
    affiliation_ordinal INTEGER NOT NULL CHECK (affiliation_ordinal > 0),
    position            TEXT CHECK (octet_length(position) >= 1),
    created_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT diesel_manage_updated_at('affiliation');

CREATE TABLE affiliation_history (
    affiliation_history_id   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    affiliation_id           UUID NOT NULL REFERENCES affiliation(affiliation_id) ON DELETE CASCADE,
    account_id               UUID NOT NULL REFERENCES account(account_id),
    data                     JSONB NOT NULL,
    timestamp                TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create institution entries for every existing contribution institution
-- (unless an institution with that name already exists).
INSERT INTO institution(institution_name)
    SELECT DISTINCT institution FROM contribution
        WHERE institution IS NOT NULL
        AND NOT EXISTS (SELECT * FROM institution WHERE institution_name = contribution.institution);

-- Create an affiliation linking the appropriate institution to each relevant contribution.
-- (Each contribution will have a maximum of one institution, so all entries can have ordinal 1.)
INSERT INTO affiliation(contribution_id, institution_id, affiliation_ordinal)
    SELECT contribution.contribution_id, institution.institution_id, 1 FROM contribution, institution
        WHERE contribution.institution = institution.institution_name;

ALTER TABLE contribution
    DROP COLUMN institution;
