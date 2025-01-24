-- Your SQL goes here
CREATE TABLE `sports_club` (
  `vat_number` varchar(11) NOT NULL COMMENT 'Partita IVA della società sportiva',
  `name` varchar(64) NOT NULL,
  `address` varchar(64) DEFAULT NULL,
  `city` varchar(64) DEFAULT NULL,
  `phone` varchar(64) DEFAULT NULL,
  PRIMARY KEY (`vat_number`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `camera` (
  `id` bigint(20) NOT NULL,
  `ipv4_address` varchar(15) NOT NULL,
  `ipv6_address` varchar(39) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `ipv4_address` (`ipv4_address`),
  UNIQUE KEY `ipv6_address` (`ipv6_address`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `rfid_tag` (
  `id` bigint(20) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `coach_type` (
  `name` varchar(64) NOT NULL,
  PRIMARY KEY (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `person` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `name` varchar(64) NOT NULL,
  `surname` varchar(64) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Rappresenta una qualsiasi persona che è necessario sia presente nel sistema. NB: questa tabella è diversa da "user" perchè qui possono essere inserite persone (ad esempio giocatori) senza la necessità che creino un account per usare il sistema';

CREATE TABLE `administrator` (
  `person_id` bigint(20) NOT NULL,
  PRIMARY KEY (`person_id`),
  CONSTRAINT `administrator_person_id_fk` FOREIGN KEY (`person_id`) REFERENCES `person` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `player` (
  `person_id` bigint(20) NOT NULL,
  PRIMARY KEY (`person_id`),
  CONSTRAINT `player_person_id_fk` FOREIGN KEY (`person_id`) REFERENCES `person` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `coach` (
  `person_id` bigint(20) NOT NULL,
  `role` varchar(64) NOT NULL,
  PRIMARY KEY (`person_id`),
  KEY `coach_role_fk` (`role`),
  CONSTRAINT `coach_person_id_fk` FOREIGN KEY (`person_id`) REFERENCES `person` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `coach_role_fk` FOREIGN KEY (`role`) REFERENCES `coach_type` (`name`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `fan` (
  `person_id` bigint(20) NOT NULL,
  PRIMARY KEY (`person_id`),
  CONSTRAINT `fan_person_id_fk` FOREIGN KEY (`person_id`) REFERENCES `person` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `user_invitation` (
  `access_code` char(10) NOT NULL COMMENT 'Codice univoco da fornire al sistema per usare questo invito in fase di registrazione (il codice può essere comunicato alla persona tramite email automatica oppure autonomamente da chi ha inserito l''invito)',
  `person_id` bigint(20) NOT NULL COMMENT 'ID della persona alla quale verrà associato il nuovo utente che si registrerà con questo invito',
  `email` varchar(320) DEFAULT NULL COMMENT 'Se viene inserita anche l''email nell''invito, il sistema può controllare che l''email inserita in fase di registrazione corrisponda a quella dell''invito',
  PRIMARY KEY (`access_code`,`person_id`),
  KEY `user_invitation_person_id_fk` (`person_id`),
  CONSTRAINT `user_invitation_person_id_fk` FOREIGN KEY (`person_id`) REFERENCES `person` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Rappresenta un invito per una certa persona ad usare il sistema. L''''utente'' che inserisce la ''persona'' nel sistema (ad esempio un allenatore che inserisce i suoi giocatori) può decidere di inserire un ''invito'' che tale persona potrà usare per creare un ''utente'' direttamente collegato alla ''persona'' già inserita nel database. Se viene inserita anche l''email nell''invito, il sistema può controllare che l''email inserita in fase di registrazione corrisponda a quella dell''invito';

CREATE TABLE `user` (
  `person_id` bigint(20) NOT NULL COMMENT 'ID della persona alla quale questo utente è associato',
  `email` varchar(320) NOT NULL,
  `password` varchar(255) NOT NULL,
  `birth_date` date DEFAULT NULL,
  `address` varchar(64) DEFAULT NULL,
  `city` varchar(64) DEFAULT NULL,
  `phone` varchar(64) DEFAULT NULL,
  `profile_image_location` varchar(255) NOT NULL COMMENT 'Posizione dell''immagine del profilo di questo utente',
  `verified` tinyint(1) NOT NULL COMMENT 'Se questo utente è stato verificato tramite email',
  `signup_datetime` datetime NOT NULL COMMENT 'Data di registrazione dell''utente nel sistema',
  PRIMARY KEY (`person_id`),
  UNIQUE KEY `email` (`email`),
  CONSTRAINT `user_person_id_fk` FOREIGN KEY (`person_id`) REFERENCES `person` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Rappresenta un utilizzatore del sistema (con le informazioni per l''accesso). Un utente deve essere  associato ad una persona. La differenza tra le due tabelle è che persona permette di inserire nel sistema un nome e cognome (ad esempio giocatori) senza la necessità di creare un account per usare il sistema';

CREATE TABLE `user_club` (
  `user_id` bigint(20) NOT NULL,
  `club_id` varchar(11) NOT NULL,
  `since_date` datetime NOT NULL,
  `until_date` datetime DEFAULT NULL,
  PRIMARY KEY (`user_id`,`club_id`,`since_date`),
  KEY `user_club_club_id_fk` (`club_id`),
  CONSTRAINT `user_club_club_id_fk` FOREIGN KEY (`club_id`) REFERENCES `sports_club` (`vat_number`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `user_club_user_id_fk` FOREIGN KEY (`user_id`) REFERENCES `user` (`person_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Rappresenta il collegamento tra una società sportiva e l''utente che la rappresenta/gestisce nel sistema';

CREATE TABLE `sport` (
  `name` varchar(64) NOT NULL,
  PRIMARY KEY (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `booking` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `author_id` bigint(20) NOT NULL COMMENT 'Utente che ha inserito la prenotazione nel sistema',
  `start_datetime` datetime NOT NULL COMMENT 'Inizio della prenotazione, ovvero del periodo di tempo nel quale il campo viene usato',
  `end_datetime` datetime NOT NULL COMMENT 'Fine della prenotazione, ovvero del periodo di tempo nel quale il campo viene usato',
  `sport` varchar(64) NOT NULL,
  `notes` text DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `booking_author_id_fk` (`author_id`),
  KEY `booking_sport_fk` (`sport`),
  CONSTRAINT `booking_author_id_fk` FOREIGN KEY (`author_id`) REFERENCES `user` (`person_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `booking_sport_fk` FOREIGN KEY (`sport`) REFERENCES `sport` (`name`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `team` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `name` varchar(64) NOT NULL,
  `club_id` varchar(11) NOT NULL COMMENT 'ID della società sportiva alla quale fa parte questa squadra',
  `sport` varchar(64) NOT NULL COMMENT 'Sport praticato da questa squadra',
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`,`club_id`),
  KEY `team_sport_fk` (`sport`),
  KEY `team_club_id_fk` (`club_id`),
  CONSTRAINT `team_club_id_fk` FOREIGN KEY (`club_id`) REFERENCES `sports_club` (`vat_number`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `team_sport_fk` FOREIGN KEY (`sport`) REFERENCES `sport` (`name`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `coach_team` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `coach_id` bigint(20) NOT NULL,
  `team_id` bigint(20) NOT NULL,
  `since_date` datetime NOT NULL COMMENT 'Quando il coach ha iniziato a lavorare per il team',
  `until_date` datetime DEFAULT NULL COMMENT 'Quando il coach ha smesso di lavorare per il team (se è null, il coach sta ancora lavorando per questo team)',
  PRIMARY KEY (`id`),
  UNIQUE KEY `coach_id` (`coach_id`,`team_id`,`since_date`),
  KEY `coach_team_team_id_fk` (`team_id`),
  CONSTRAINT `coach_team_coach_id_fk` FOREIGN KEY (`coach_id`) REFERENCES `coach` (`person_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `coach_team_team_id_fk` FOREIGN KEY (`team_id`) REFERENCES `team` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Memorizza per quale team lavora o ha lavorato un coach (nel caso stia ancora lavorando per questo team, il campo until_date è null)';

CREATE TABLE `player_team` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `player_id` bigint(20) NOT NULL,
  `team_id` bigint(20) NOT NULL,
  `since_date` datetime NOT NULL COMMENT 'Quando il giocatore ha iniziato a giocare per il team',
  `until_date` datetime DEFAULT NULL COMMENT 'Quando il giocatore ha smesso di giocare per il team (se è null, il giocatore sta ancora giocando per questo team)',
  PRIMARY KEY (`id`),
  UNIQUE KEY `player_id` (`player_id`,`team_id`,`since_date`),
  KEY `player_team_team_id_fk` (`team_id`),
  CONSTRAINT `player_team_player_id_fk` FOREIGN KEY (`player_id`) REFERENCES `player` (`person_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `player_team_team_id_fk` FOREIGN KEY (`team_id`) REFERENCES `team` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Memorizza per quale team gioca o ha giocato un giocatore (nel caso stia ancora giocando per questo team, il campo until_date è null)';

CREATE TABLE `influxdb_bucket` (
  `location` varchar(255) NOT NULL,
  `team_id` bigint(20) NOT NULL,
  `name` varchar(64) NOT NULL,
  `token` varchar(255) NOT NULL,
  `org` varchar(255) NOT NULL,
  `db` varchar(255) NOT NULL,
  PRIMARY KEY (`location`),
  KEY `influxdb_bucket_team_id_fk` (`team_id`),
  CONSTRAINT `influxdb_bucket_team_id_fk` FOREIGN KEY (`team_id`) REFERENCES `team` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Riferimento a dove vengono memorizzate le informazioni ottenute dai sensori';

CREATE TABLE `training` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `team_id` bigint(20) NOT NULL,
  `start_datetime` datetime NOT NULL COMMENT 'Data e ora di inizio dell''allenamento (NB: questo valore può differire dalla data di inizio della prenotazione a cui questo allenamento è associato)',
  `end_datetime` datetime DEFAULT NULL COMMENT 'Data e ora di fine dell''allenamento (NB: questo valore può differire dalla data di fine della prenotazione a cui questo allenamento è associato)',
  `booking_id` bigint(20) NOT NULL COMMENT 'ID della prenotazione che è stata inserita per riservare il campo per questo allenamento',
  PRIMARY KEY (`id`),
  UNIQUE KEY `booking_id` (`booking_id`),
  KEY `training_team_id_fk` (`team_id`),
  CONSTRAINT `training_booking_id_fk` FOREIGN KEY (`booking_id`) REFERENCES `booking` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `training_team_id_fk` FOREIGN KEY (`team_id`) REFERENCES `team` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Rappresenta un allenamento di una singola squadra';

CREATE TABLE `training_player` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `training_id` bigint(20) NOT NULL,
  `player_id` bigint(20) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `training_id` (`training_id`,`player_id`),
  KEY `training_player_player_id_fk` (`player_id`),
  CONSTRAINT `training_player_player_id_fk` FOREIGN KEY (`player_id`) REFERENCES `player` (`person_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `training_player_training_id_fk` FOREIGN KEY (`training_id`) REFERENCES `training` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Memorizza la presenza o meno di un giocatore in un dato allenamento';

CREATE TABLE `training_player_tag` (
  `training_id` bigint(20) NOT NULL,
  `player_id` bigint(20) NOT NULL,
  `rfid_tag_id` bigint(20) NOT NULL,
  PRIMARY KEY (`training_id`,`player_id`,`rfid_tag_id`),
  KEY `training_player_tag_rfid_tag_id_fk` (`rfid_tag_id`),
  KEY `training_player_tag_player_id_fk` (`player_id`),
  CONSTRAINT `training_player_tag_player_id_fk` FOREIGN KEY (`player_id`) REFERENCES `player` (`person_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `training_player_tag_rfid_tag_id_fk` FOREIGN KEY (`rfid_tag_id`) REFERENCES `rfid_tag` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `training_player_tag_training_id_fk` FOREIGN KEY (`training_id`) REFERENCES `training` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Per ogni giocatore in un allenamento, permette di associare molteplici sensori ad un giocatore';

CREATE TABLE `formation` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `team_id` bigint(20) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `formation_team_id_fk` (`team_id`),
  CONSTRAINT `formation_team_id_fk` FOREIGN KEY (`team_id`) REFERENCES `team` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Rappresenta una formazione di giocatori da mettere in campo per una partita';

CREATE TABLE `game` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `home_formation_id` bigint(20) NOT NULL COMMENT 'ID della formazione della squadra che gioca in casa',
  `visiting_formation_id` bigint(20) DEFAULT NULL COMMENT 'ID della formazione della squadra che gioca in trasferta (Nel caso in cui il team in trasferta non faccia uso del sistema (e quindi non è registrato), il campo è null)',
  `start_datetime` datetime NOT NULL COMMENT 'Data e ora di inizio della partita (NB: questo valore può differire dalla data di inizio della prenotazione a cui questa partita è associata)',
  `end_datetime` datetime DEFAULT NULL COMMENT 'Data e ora di fine della partita (NB: questo valore può differire dalla data di fine della prenotazione a cui questa partita è associata)',
  `booking_id` bigint(20) NOT NULL COMMENT 'ID della prenotazione che è stata inserita per riservare il campo per questa partita',
  PRIMARY KEY (`id`),
  UNIQUE KEY `booking_id` (`booking_id`),
  KEY `game_home_formation_id_fk` (`home_formation_id`),
  KEY `game_visiting_formation_id_fk` (`visiting_formation_id`),
  CONSTRAINT `game_booking_id_fk` FOREIGN KEY (`booking_id`) REFERENCES `booking` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `game_home_formation_id_fk` FOREIGN KEY (`home_formation_id`) REFERENCES `formation` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `game_visiting_formation_id_fk` FOREIGN KEY (`visiting_formation_id`) REFERENCES `formation` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Rappresenta una partita tra due formazioni (una di casa e una in trasferta). Nel caso in cui il team in trasferta non faccia uso del sistema (e quindi non è registrato), il campo per la formazione in trasferta è null';

CREATE TABLE `formation_player` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `formation_id` bigint(20) NOT NULL,
  `player_id` bigint(20) NOT NULL,
  `starting` tinyint(1) NOT NULL COMMENT 'Se il giocatore è un titolare in questa formazione',
  `entry_minute` time DEFAULT NULL COMMENT 'Minuto della partita nel quale il giocatore è entrato in campo',
  `exit_minute` time DEFAULT NULL COMMENT 'Minuto della partita nel quale il giocatore è uscito dal campo',
  PRIMARY KEY (`id`),
  UNIQUE KEY `formation_id` (`formation_id`,`player_id`),
  KEY `formation_player_player_id_fk` (`player_id`),
  CONSTRAINT `formation_player_formation_id_fk` FOREIGN KEY (`formation_id`) REFERENCES `formation` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `formation_player_player_id_fk` FOREIGN KEY (`player_id`) REFERENCES `player` (`person_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Memorizza la presenza o meno di un giocatore in una formazione di gioco';

CREATE TABLE `formation_player_tag` (
  `formation_id` bigint(20) NOT NULL,
  `player_id` bigint(20) NOT NULL,
  `rfid_tag_id` bigint(20) NOT NULL,
  PRIMARY KEY (`formation_id`,`player_id`,`rfid_tag_id`),
  KEY `formation_player_tag_rfid_tag_id_fk` (`rfid_tag_id`),
  KEY `formation_player_tag_player_id_fk` (`player_id`),
  CONSTRAINT `formation_player_tag_formation_id_fk` FOREIGN KEY (`formation_id`) REFERENCES `formation` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `formation_player_tag_player_id_fk` FOREIGN KEY (`player_id`) REFERENCES `player` (`person_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `formation_player_tag_rfid_tag_id_fk` FOREIGN KEY (`rfid_tag_id`) REFERENCES `rfid_tag` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Per ogni giocatore in una formazione, permette di associare molteplici sensori ad un giocatore';

CREATE TABLE `recording_session` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `author_id` bigint(20) NOT NULL COMMENT 'Utente che ha programmato/avviato la sessione di registrazione',
  `start_datetime` datetime NOT NULL COMMENT 'Data e ora di inizio della registrazione (NB: questo valore può differire dalla data di inizio della prenotazione a cui questa sessione è associata)',
  `end_datetime` datetime NOT NULL COMMENT 'Data e ora di fine della registrazione (NB: questo valore può differire dalla data di fine della prenotazione a cui questa sessione è associata)',
  `booking_id` bigint(20) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `recording_session_author_id_fk` (`author_id`),
  KEY `recording_session_booking_id_fk` (`booking_id`),
  CONSTRAINT `recording_session_author_id_fk` FOREIGN KEY (`author_id`) REFERENCES `user` (`person_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `recording_session_booking_id_fk` FOREIGN KEY (`booking_id`) REFERENCES `booking` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Una programmazione di utilizzo delle telecamere associata a una prenotazione';

CREATE TABLE `camera_session` (
  `session_id` bigint(20) NOT NULL,
  `camera_id` bigint(20) NOT NULL,
  PRIMARY KEY (`session_id`,`camera_id`),
  KEY `camera_session_camera_id_fk` (`camera_id`),
  CONSTRAINT `camera_session_camera_id_fk` FOREIGN KEY (`camera_id`) REFERENCES `camera` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `camera_session_session_id_fk` FOREIGN KEY (`session_id`) REFERENCES `recording_session` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Memorizza quali telecamere sono state selezionate per essere utilizzate in una sessione di registrazione';

CREATE TABLE `video` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `file_location` varchar(255) NOT NULL,
  `name` varchar(64) NOT NULL,
  `date` datetime NOT NULL,
  `notes` text DEFAULT NULL,
  `session_id` bigint(20) NOT NULL COMMENT 'ID della sessione di registrazione che ha portato alla creazione di questo video',
  `camera_id` bigint(20) NOT NULL COMMENT 'ID della camera che ha registrato questo video',
  PRIMARY KEY (`id`),
  UNIQUE KEY `file_location` (`file_location`),
  KEY `video_session_id_fk` (`session_id`),
  KEY `video_camera_id_fk` (`camera_id`),
  CONSTRAINT `video_camera_id_fk` FOREIGN KEY (`camera_id`) REFERENCES `camera` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `video_session_id_fk` FOREIGN KEY (`session_id`) REFERENCES `recording_session` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `time_marker` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `instant` time(3) NOT NULL,
  `video_id` bigint(20) NOT NULL COMMENT 'ID del video nel quale è stato inserito questo marker',
  `name` varchar(64) NOT NULL,
  `notes` text DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `time_marker_video_id_fk` (`video_id`),
  CONSTRAINT `time_marker_video_id_fk` FOREIGN KEY (`video_id`) REFERENCES `video` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `screenshot` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `file_location` varchar(255) NOT NULL,
  `video_id` bigint(20) NOT NULL COMMENT 'ID del video dal quale è stato estratto questo screenshot',
  `instant` time(3) NOT NULL COMMENT 'Istante del video nel quale è stato estratto questo screenshot',
  `name` varchar(64) NOT NULL,
  `notes` text DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `file_location` (`file_location`),
  KEY `screenshot_video_id_fk` (`video_id`),
  CONSTRAINT `screenshot_video_id_fk` FOREIGN KEY (`video_id`) REFERENCES `video` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `video_user` (
  `user_id` bigint(20) NOT NULL,
  `video_id` bigint(20) NOT NULL,
  `is_owner` tinyint(1) NOT NULL,
  `read` tinyint(1) NOT NULL,
  `edit` tinyint(1) NOT NULL,
  `delete` tinyint(1) NOT NULL,
  `share` tinyint(1) NOT NULL,
  PRIMARY KEY (`user_id`,`video_id`),
  KEY `video_user_video_id_fk` (`video_id`),
  CONSTRAINT `video_user_user_id_fk` FOREIGN KEY (`user_id`) REFERENCES `user` (`person_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `video_user_video_id_fk` FOREIGN KEY (`video_id`) REFERENCES `video` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Rappresenta la relazione tra un certo utente e un video, dove i vari parametri booleani specificano le azioni possibili per l''utente su questo video';

CREATE TABLE `clip_video` (
  `original_video_id` bigint(20) NOT NULL COMMENT 'ID del video dal quale la clip è stata estratta',
  `clip_id` bigint(20) NOT NULL COMMENT 'ID della clip estratta',
  PRIMARY KEY (`original_video_id`,`clip_id`),
  KEY `clip_video_clip_id_fk` (`clip_id`),
  CONSTRAINT `clip_video_clip_id_fk` FOREIGN KEY (`clip_id`) REFERENCES `video` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `clip_video_original_video_id_fk` FOREIGN KEY (`original_video_id`) REFERENCES `video` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='Memorizza il video dal quale proviene un altro video, dove quest''ultimo è una clip di un altro video';

